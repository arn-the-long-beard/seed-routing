mod model;
mod path;
mod url;
mod view;
use seed::Url;
use std::fmt::Debug;

pub use model::*;
pub use path::*;
pub use url::*;
pub use view::*;

// ------ ------
//     Urls
// ------ ------
use seed::*;

struct_urls!();
/// Construct url injected in the web browser with path
impl<'a> Urls<'a> {
    pub fn build_url(self, segments: Vec<&str>) -> Url {
        self.base_url().set_path(segments)
    }
}
// pub mod children;
// pub mod route;
pub enum Move {
    IsNavigating,
    IsMovingBack,
    IsMovingForward,
    IsReady,
}

/// Router that management navigation between routes
/// Store the history
/// Can go back and forward
/// Manage the default route
pub struct Router<Routes: Debug + PartialEq + ParsePath + Clone + Default + Navigation> {
    /// The actual route , which should be the one displaying the view in Seed
    pub current_route: Option<Routes>,
    /// The index of the history
    /// It will change when navigation or pushing back or forward
    pub current_history_index: usize,
    /// The default route extracted from the attribute #[default_route] on your
    /// enum This route is equivalent to 404 . In other web framework it
    /// would be matching path pattern "*" for example
    pub default_route: Routes,
    /// The route url of the route
    /// ∕∕todo add protocol, domain and extrat info later
    base_url: Url,
    /// The current operation the router is doing
    pub current_move: Move,
    /// The full history with all the routes the user has visited
    history: Vec<Routes>,
}

impl<Routes: Debug + PartialEq + Default + ParsePath + Clone + Navigation> Default
    for Router<Routes>
{
    fn default() -> Self {
        Router {
            current_history_index: 0,
            default_route: Routes::default(),
            history: Vec::new(),
            current_route: None,
            base_url: Url::new(), // should replace with current ,maybe ?
            current_move: Move::IsReady,
        }
    }
}

impl<Routes: Debug + PartialEq + ParsePath + Default + Clone + Navigation> Router<Routes> {
    /// Same as default for now
    pub fn new() -> Router<Routes> {
        Router::default()
    }

    /// set base url, useless for now
    pub fn set_base_url(&mut self, url: Url) -> &mut Self {
        self.base_url = url;
        self
    }

    /// Init navigation with the given url
    pub fn init_url_and_navigation(&mut self, url: Url) -> &mut Self {
        self.set_base_url(url.to_base_url());
        self.navigate_to_url(url);
        self
    }
    // pub fn routes_values(&'static self) -> Values<String> {
    //     let mut values = &self.routes.values();
    //     values.clone()
    // }
    // pub fn add_route(&mut self, route: Routes, value: &str) -> &mut Self {
    //     self.routes.insert(value.to_string(), route);
    //     self
    // }

    /// Push the route to the history so you can go back to it later
    fn push_to_history(&mut self, route: Routes) {
        self.history.push(route);
        self.current_history_index = self.history.len() - 1;
    }

    /// Go back in history and navigate back to the previous route
    ///  # Note for now it does not add to history since we navigate inside
    pub fn back(&mut self) -> bool {
        if let Some(next_route) = self.can_back_with_route() {
            self.current_route = Some(next_route);
            self.current_history_index -= 1;
            true
        } else {
            false
        }
    }

    /// Check if you can go back in history and give you the right route
    pub fn can_back_with_route(&self) -> Option<Routes> {
        // If we have no history, cannot go back

        if self.history.is_empty() {
            return None;
        }
        // If the current route is at index 0, we cannot go back more
        if self.current_history_index == 0 {
            return None;
        }
        let next_index = &self.current_history_index - 1;
        let route = self.history.get(next_index).unwrap();
        Some(route.clone())
    }

    pub fn can_back(&self) -> bool {
        self.can_back_with_route().is_some()
    }
    pub fn can_forward(&self) -> bool {
        self.can_forward_with_route().is_some()
    }

    /// Check if you can navigate forward in the history
    pub fn can_forward_with_route(&self) -> Option<Routes> {
        // if there is no route, cannot go forward
        if self.history.is_empty() {
            return None;
        }
        // If we are on the last index, we cannot go forward neither
        if self.current_history_index == self.history.len() - 1 {
            return None;
        }
        let next_index = &self.current_history_index + 1;

        let route = self.history.get(next_index).unwrap_or_else(|| {
            panic!(
                "We should have get route but index is failed {}",
                next_index
            )
        });
        Some(route.clone())
    }

    /// to move forward in the history
    /// # Note for now it does not add to history since we navigate inside
    pub fn forward(&mut self) -> bool {
        if let Some(next_route) = self.can_forward_with_route() {
            self.current_route = Some(next_route);
            self.current_history_index += 1;
            true
        } else {
            false
        }
    }

    /// Check the route is the current route
    /// Could be use directly with url as well
    pub fn is_current_route(&self, route: &Routes) -> bool {
        if let Some(current_route) = &self.current_route {
            route.eq(&current_route)
        } else {
            false
        }
    }

    /// Go to the next url with the associated route
    /// This will push to history. So If you go back multiple time and then use
    /// navigate and then go back, you will not get the previous page, but the
    /// one just pushed into history before
    pub fn navigate_to_new(&mut self, route: &Routes) {
        self.current_route = Some(route.clone());
        self.push_to_history(route.clone());
    }

    /// Match the url that change and update the router with the new current
    /// Route
    pub fn navigate_to_url(&mut self, url: Url) {
        if let Ok(route_match) = Routes::from_url(url) {
            // log!("found route");
            self.navigate_to_new(&route_match);
        } else {
            self.navigate_to_new(&self.default_route.clone());
        }
    }

    /// Ask Seed the new request url back in history
    pub fn request_moving_back<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
        self.current_move = Move::IsMovingBack;
        if let Some(next_route) = &self.can_back_with_route() {
            func(next_route.to_url());
        }
    }

    /// Ask Seed the new request url forward in history
    pub fn request_moving_forward<F: FnOnce(Url) -> R, R>(&mut self, func: F) {
        self.current_move = Move::IsMovingForward;
        if let Some(next_route) = &self.can_forward_with_route() {
            func(next_route.to_url());
        }
    }
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// This method accept a given url and choose the appropriate update for the
    /// history
    /// It also reset the current move to Ready
    /// This tiny lifecycle is useless because we can know if we had be going
    /// back in time or forward or notmal navigation
    pub fn confirm_navigation(&mut self, url: Url) {
        match self.current_move {
            Move::IsNavigating => {
                self.navigate_to_url(url);
            }
            Move::IsMovingBack => {
                self.back();
            }
            Move::IsMovingForward => {
                self.forward();
            }
            Move::IsReady => {
                self.navigate_to_url(url);
            }
        }
        self.current_move = Move::IsReady;
    }
}

#[cfg(test)]
mod test {
    use seed::Url;

    extern crate router_macro_derive;
    use super::*;
    use crate::{router, ParseError};
    use router_macro_derive::{AsUrl, Root};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Copy, Clone, AsUrl)]
    pub enum DashboardAdminRoutes {
        Other,
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, AsUrl)]
    pub enum DashboardRoutes {
        Admin(DashboardAdminRoutes),
        Profile(u32),
        #[as_path = ""]
        Root,
    }

    #[derive(Debug, PartialEq, Clone, AsUrl, Root)]
    enum ExampleRoutes {
        Login,
        Register,
        Stuff,
        Dashboard(DashboardRoutes),
        #[default_route]
        NotFound,
        #[as_path = ""]
        Home,
    }

    #[wasm_bindgen_test]
    fn test_router_default_route() {
        let mut router = Router::<ExampleRoutes>::new();
        let url = Url::new().add_path_part("example");
        router.navigate_to_url(url);
        assert_eq!(router.current_route.unwrap(), router.default_route);
    }

    #[wasm_bindgen_test]
    fn test_build_url() {
        let mut router: Router<ExampleRoutes> = Router::new();
        let url = router.base_url().clone().add_path_part("");
        router.navigate_to_url(url);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("").unwrap()
        );

        let admin_url = router
            .base_url()
            .clone()
            .set_path("dashboard/admin/other".split('/'));

        router.navigate_to_url(admin_url);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("/dashboard/admin/other").unwrap()
        );

        let admin_url = router
            .base_url()
            .clone()
            .set_path("dashboard/profile/1".split('/'));

        router.navigate_to_url(admin_url);
        assert_eq!(
            router.current_route.unwrap(),
            ExampleRoutes::parse_path("/dashboard/profile/1").unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_navigation_to_route() {
        let mut router: Router<ExampleRoutes> = Router::new();
        router.navigate_to_new(&ExampleRoutes::parse_path("/dashboard/profile/1").unwrap());

        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
        );
        assert_eq!(router.current_history_index, 0);

        router.navigate_to_new(&ExampleRoutes::parse_path("/dashboard/profile/55").unwrap());

        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(55))
        );
        assert_eq!(router.current_history_index, 1);
        router.navigate_to_new(&ExampleRoutes::Home);

        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("").unwrap()
        );
        assert_eq!(router.current_history_index, 2);
    }

    #[wasm_bindgen_test]
    fn test_backward() {
        let mut router: Router<ExampleRoutes> = Router::new();

        let back = router.back();
        assert_eq!(back, false, "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index, 0,
            "We should have current index 0"
        );
        assert_eq!(
            router.current_route.is_none(),
            true,
            "We should not have current rou"
        );

        router.navigate_to_new(&ExampleRoutes::parse_path("").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("register").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("dashboard/admin/other").unwrap());

        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Register
        );
        assert_eq!(router.is_current_route(&ExampleRoutes::Register), true);
        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 0);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::parse_path("").unwrap()
        );

        router.navigate_to_new(&ExampleRoutes::Dashboard(DashboardRoutes::Root));
        assert_eq!(
            router.is_current_route(&ExampleRoutes::parse_path("dashboard/").unwrap()),
            true
        );

        let back = router.back();
        assert_eq!(back, true);
        // Here is tricky part, after navigate we go back to the end of history, so if
        // we go back, we go to the previous index
        assert_eq!(router.current_history_index, 2);
        assert_eq!(
            router.current_route.unwrap(),
            ExampleRoutes::parse_path("dashboard/admin/other").unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_forward() {
        let mut router: Router<ExampleRoutes> = Router::new();

        let back = router.forward();
        assert_eq!(back, false, "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index, 0,
            "We should have current index 0"
        );
        assert_eq!(
            router.current_route.is_none(),
            true,
            "We should not have current rou"
        );
        router.navigate_to_new(&ExampleRoutes::parse_path("").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("register").unwrap());
        router.navigate_to_new(&ExampleRoutes::parse_path("/dashboard/profile/55").unwrap());
        assert_eq!(router.current_history_index, 2);

        let _ = router.back();
        let _ = router.back();

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Register
        );

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 2);
        assert_eq!(
            router.current_route.clone().unwrap(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(55))
        );
        let forward = router.forward();
        assert_eq!(forward, false, "We should Not have gone forward");
    }
}
