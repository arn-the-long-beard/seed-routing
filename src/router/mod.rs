mod default_route;
mod model;
mod path;
mod url;
mod view;
use seed::Url;
use std::fmt::Debug;

pub use default_route::*;
pub use model::*;
pub use path::*;
use seed::prelude::{
    wasm_bindgen::__rt::std::{cell::RefCell, rc::Rc},
    SubHandle,
};
pub use url::*;
pub use view::*;

// pub mod children;
// pub mod route;
#[derive(Clone, Debug, PartialEq)]
pub enum MoveStatus {
    Navigating,
    MovingBack,
    MovingForward,
    Ready,
}

/// The contained data inside the Router.
#[allow(clippy::module_name_repetitions)]
pub struct RouterData<Route: Debug + PartialEq + ParsePath + Clone + Default + Navigation> {
    /// The actual route, which should be the one displaying the view in Seed.
    pub current_route: Route,
    /// The index of the history.
    /// It will change when navigation or pushing back or forward.
    pub current_history_index: usize,
    /// The default route extracted from the attribute #[default_route] on your
    /// enum. This route is equivalent to 404. In other web framework it
    /// would be matching path pattern "*" for example.
    pub default_route: Route,
    /// The route url of the route
    /// ∕∕todo add protocol, domain and extract info later.
    base_url: Url,
    /// The current operation the router is doing.
    pub current_move: MoveStatus,

    pub sub_handle: Option<SubHandle>,
    /// The full history with all the routes the user has visited.
    history: Vec<Route>,
}

impl<Route: Debug + PartialEq + ParsePath + Clone + Default + Navigation> RouterData<Route> {
    /// Add the route to the history
    pub fn push_to_history(&mut self, route: Route) {
        self.history.push(route);
        self.current_history_index = self.history.len() - 1;
    }
    /// Check the current page is the last one in the history
    pub fn is_on_last_index(&self) -> bool {
        self.current_history_index == self.history.len() - 1
    }
}

/// Router that manages navigation between routes,
/// Store the history,
/// Can go back and forward,
/// Manage the default route.
#[derive(Clone)]
pub struct Router<Route: Debug + PartialEq + ParsePath + Clone + Default + Navigation> {
    data: Rc<RefCell<RouterData<Route>>>,
}

impl<Route: 'static + Debug + PartialEq + ParsePath + Default + Clone + Navigation> Router<Route> {
    /// Create a new Router with no url, no history and current route is default
    /// route
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(RouterData {
                current_history_index: 0,
                default_route: Route::default(),
                history: Vec::new(),
                current_route: Route::default(),
                base_url: Url::new(), // should replace with current ,maybe ?
                current_move: MoveStatus::Ready,
                sub_handle: None,
            })),
        }
    }

    fn update_data(&self, updater: impl FnOnce(&mut RouterData<Route>)) {
        updater(&mut self.data.borrow_mut());
    }

    fn map_data<T>(&self, mapper: impl FnOnce(&RouterData<Route>) -> T) -> T {
        mapper(&self.data.borrow())
    }

    /// Init navigation with the given url.
    pub fn init(&self, url: Url) -> &Self {
        self.set_base_url(&url);
        self.navigate_to_url(url);
        self
    }

    /// Set the root URL which all paths are built from.
    fn set_base_url(&self, url: &Url) -> &Self {
        self.update_data(|data| data.base_url = url.to_base_url());
        self
    }

    /// Register a subscribe handle to confirm navigation when Url Requested.
    pub fn subscribe(&self, sub_handle: SubHandle) -> &Self {
        self.clone()
            .update_data(|data| data.sub_handle = Some(sub_handle));
        self
    }

    /// Push the route to the history so you can go back to it later.
    fn push_to_history(&self, route: Route) {
        self.update_data(|data| data.push_to_history(route));
    }

    /// If a previous `Route` in history exists, return it. Otherwise return `None`
    pub fn peek_back(&self) -> Option<Route> {
        // If we have no history, cannot go back

        if self.map_data(|data| data.history.is_empty()) {
            return None;
        }
        // If the current route is at index 0, we cannot go back more
        if self.map_data(|data| data.current_history_index) == 0 {
            return None;
        }
        let next_index: usize = self.map_data(|data| data.current_history_index - 1);
        let history = &self.map_data(|data| data.history.clone());
        let route = history.get(next_index).unwrap();
        Some(route.clone())
    }

    /// If a next `Route` in history exists, return it. Otherwise return `None`
    pub fn peek_forward(&self) -> Option<Route> {
        // if there is no route, cannot go forward
        if self.map_data(|data| data.history.is_empty()) {
            return None;
        }
        // If we are on the last index, we cannot go forward neither
        if self.map_data(RouterData::is_on_last_index) {
            return None;
        }
        let next_index = self.map_data(|data| data.current_history_index + 1);

        let history = &self.map_data(|data| data.history.clone());

        let route = history.get(next_index).unwrap_or_else(|| {
            panic!(
                "We should have get route but index is failed {}",
                next_index
            )
        });

        Some(route.clone())
    }

    /// Same as [Router::peek_back], with the addition of navigating to a resulting `Some(Route)`
    ///
    ///  # Note for now it does not add to history since we navigate inside
    pub fn back(&self) -> Option<Route> {
        self.peek_back().map(|next_route| {
            self.set_current_route(&next_route);
            self.update_data(|data| data.current_history_index -= 1);
            next_route
        })
    }

    /// Same as [Router::peek_forward], with the addition of navigating to a resulting `Some(Route)`
    ///
    /// # Note for now it does not add to history since we navigate inside
    pub fn forward(&self) -> Option<Route> {
        self.peek_forward().map(|next_route| {
            self.set_current_route(&next_route);
            self.update_data(|data| data.current_history_index += 1);
            next_route
        })
    }

    /// Check the route is the current route.
    /// Could be use directly with url as well.
    pub fn is_current_route(&self, route: &Route) -> bool {
        route.eq(&self.current_route())
    }

    /// Go to the next url with the associated route.
    /// This will push to history. So If you go back multiple time and then use
    /// navigate and then go back, you will not get the previous page, but the
    /// one just pushed into history before.
    pub fn navigate_to_new(&self, route: Route) {
        self.set_current_route(&route);
        self.push_to_history(route);
    }

    /// Match the url that change and update the router with the new current
    /// Route.
    pub fn navigate_to_url(&self, url: Url) {
        if let Ok(route_match) = Route::from_url(url) {
            // log!("found route");
            self.navigate_to_new(route_match);
        } else {
            let default = self.default_route();
            self.navigate_to_new(default);
        }
    }

    /// Ask Seed the new request url back in history.
    pub fn request_moving_back<F: FnOnce(Url) -> R, R>(&self, func: F) {
        self.update_data(|data| data.current_move = MoveStatus::MovingBack);

        self.peek_back().map(|next_route| func(next_route.to_url()));
    }

    /// Ask Seed the new request url forward in history.
    pub fn request_moving_forward<F: FnOnce(Url) -> R, R>(&self, func: F) {
        self.update_data(|data| data.current_move = MoveStatus::MovingForward);

        self.peek_forward()
            .map(|next_route| func(next_route.to_url()));
    }

    /// This method accept a given url and choose the appropriate update for the
    /// history depending of the MoveStatus.
    /// It also resets the current move to Ready.
    pub fn confirm_navigation(&self, url: Url) {
        match self.map_data(|data| data.current_move.clone()) {
            MoveStatus::Navigating | MoveStatus::Ready => {
                self.navigate_to_url(url);
            }
            MoveStatus::MovingBack => {
                self.back();
            }
            MoveStatus::MovingForward => {
                self.forward();
            }
        }
        self.update_data(|data| data.current_move = MoveStatus::Ready);
    }

    pub fn set_current_route(&self, route: &Route) {
        self.update_data(|data| data.current_route = route.clone());
    }

    pub fn current_route(&self) -> Route {
        self.map_data(|data| data.current_route.clone())
    }

    pub fn default_route(&self) -> Route {
        self.map_data(|data| data.default_route.clone())
    }

    pub fn base_url(&self) -> Url {
        self.map_data(|data| data.base_url.clone())
    }

    pub fn current_history_index(&self) -> usize {
        self.map_data(|data| data.current_history_index)
    }
}

#[cfg(test)]
mod test {
    use seed::Url;

    extern crate router_macro_derive;

    use super::*;
    use crate::{router, ParseError};
    use router_macro_derive::{ParseUrl, Root};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Copy, Clone, ParseUrl)]
    pub enum DashboardAdminRoutes {
        Other,
        #[as_path = ""]
        Root,
    }

    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum DashboardRoutes {
        Admin(DashboardAdminRoutes),
        Profile(u32),
        #[as_path = ""]
        Root,
    }

    #[derive(Debug, PartialEq, Clone, ParseUrl, Root)]
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
    fn test_new_router() {
        let router = Router::<ExampleRoutes>::new();
        let current = router.current_route();
        let default = router.default_route();
        let router_data = router.data.borrow();

        assert_eq!(router_data.sub_handle.is_none(), true);
        assert_eq!(router_data.current_history_index, 0);
        assert_eq!(router_data.history.is_empty(), true);
        assert_eq!(router_data.current_move, MoveStatus::Ready);
        assert_eq!(router_data.current_route, current);
        assert_eq!(router_data.default_route, default);
        assert_eq!(current, default);
    }

    #[wasm_bindgen_test]
    fn test_init_router() {
        let router = Router::<ExampleRoutes>::new();
        // We should get the right route when the Url is valid
        {
            let url = ExampleRoutes::Login.to_url();
            let router = router.init(url.clone());
            let current = router.current_route();
            let router_data = router.data.borrow();
            assert_eq!(current, ExampleRoutes::Login);
            assert_eq!(router_data.base_url, url.to_base_url());
        }
        // We should get default route when the Url does not match a route
        {
            let url: Url = "http://localhost/profile".parse().unwrap();
            let router = router.init(url.clone());
            let current = router.current_route();
            let router_data = router.data.borrow();
            assert_eq!(current, ExampleRoutes::NotFound);
            assert_eq!(router_data.base_url, url.to_base_url());
        }
    }

    #[wasm_bindgen_test]
    fn test_set_base_url() {
        let router = Router::<ExampleRoutes>::new();

        let url: Url = "http://localhost/profile".parse().unwrap();
        let router = router.set_base_url(&url);
        let router_data = router.data.borrow();
        assert_eq!(router_data.base_url, "http://localhost".parse().unwrap());
    }

    #[wasm_bindgen_test]
    fn test_push_to_history() {
        let router = Router::<ExampleRoutes>::new();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Profile(23));
        router.push_to_history(route.clone());

        let history = router.map_data(|data| data.history.clone());
        assert_eq!(history.get(0).unwrap(), &route);
    }

    #[wasm_bindgen_test]
    fn test_peek_back() {
        let router = Router::<ExampleRoutes>::new();
        let previous_route = router.peek_back();
        assert_eq!(previous_route.is_none(), true);
        let route_1 = ExampleRoutes::Dashboard(DashboardRoutes::Profile(23));
        router.push_to_history(route_1.clone());
        let route_2 = ExampleRoutes::Login;
        router.push_to_history(route_2);
        let previous_route = router.peek_back().unwrap();
        assert_eq!(previous_route, route_1);
    }

    #[wasm_bindgen_test]
    fn test_router_default_route() {
        let router = Router::<ExampleRoutes>::new();
        let url = Url::new().add_path_part("example");
        router.navigate_to_url(url);

        let current = router.current_route();
        let default = router.default_route();
        assert_eq!(current, default);
    }

    #[wasm_bindgen_test]
    fn test_build_url() {
        let router: Router<ExampleRoutes> = Router::new();
        let url = router.base_url().clone().add_path_part("");
        router.navigate_to_url(url);
        assert_eq!(
            router.current_route().clone(),
            ExampleRoutes::parse_path("").unwrap()
        );

        let admin_url = router
            .base_url()
            .clone()
            .set_path("dashboard/admin/other".split('/'));

        router.navigate_to_url(admin_url);
        assert_eq!(
            router.current_route().clone(),
            ExampleRoutes::parse_path("/dashboard/admin/other").unwrap()
        );

        let admin_url = router
            .base_url()
            .clone()
            .set_path("dashboard/profile/1".split('/'));

        router.navigate_to_url(admin_url);
        assert_eq!(
            router.current_route(),
            ExampleRoutes::parse_path("/dashboard/profile/1").unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn test_navigation_to_route() {
        let router: Router<ExampleRoutes> = Router::new();
        router.navigate_to_new(ExampleRoutes::parse_path("/dashboard/profile/1").unwrap());

        assert_eq!(
            router.current_route(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(1))
        );
        assert_eq!(router.current_history_index(), 0);

        router.navigate_to_new(ExampleRoutes::parse_path("/dashboard/profile/55").unwrap());

        assert_eq!(
            router.current_route(),
            ExampleRoutes::Dashboard(DashboardRoutes::Profile(55))
        );
        assert_eq!(router.current_history_index(), 1);
        router.navigate_to_new(ExampleRoutes::Home);

        assert_eq!(
            router.current_route(),
            ExampleRoutes::parse_path("").unwrap()
        );
        assert_eq!(router.current_history_index(), 2);
    }

    // Testing return value and side effects of Router::back
    //
    // After running back, check the option it returns, and that current_path() and is_current_route() is still correct
    #[wasm_bindgen_test]
    fn test_backward() {
        let router: Router<ExampleRoutes> = Router::new();

        let back = router.back();
        assert!(back.is_none(), "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index(),
            0,
            "We should have current index 0"
        );
        router.navigate_to_new(ExampleRoutes::parse_path("").unwrap());
        router.navigate_to_new(ExampleRoutes::parse_path("register").unwrap());
        router.navigate_to_new(ExampleRoutes::parse_path("dashboard/admin/other").unwrap());

        assert_eq!(router.current_history_index(), 2);

        let back = router.back();
        assert_eq!(
            back,
            Some(ExampleRoutes::Register),
            "We should have gone backwards to \"register\""
        );
        assert_eq!(back, Some(router.current_route()));
        assert_eq!(router.current_history_index(), 1);
        assert_eq!(router.is_current_route(&ExampleRoutes::Register), true);

        let back = router.back();
        assert_eq!(
            back,
            Some(ExampleRoutes::parse_path("").unwrap()),
            "We should have gone back to root path"
        );
        assert_eq!(back, Some(router.current_route()));
        assert_eq!(router.current_history_index(), 0);

        router.navigate_to_new(ExampleRoutes::Dashboard(DashboardRoutes::Root));
        assert_eq!(
            router.is_current_route(&ExampleRoutes::parse_path("dashboard/").unwrap()),
            true
        );

        let back = router.back();
        assert_eq!(
            back,
            Some(ExampleRoutes::parse_path("dashboard/admin/other").unwrap()),
            "We should have gone back to \"dashboard/admin/other\""
        );
        assert_eq!(back, Some(router.current_route()));
        // Here is tricky part, after navigate we go back to the end of history, so if
        // we go back, we go to the previous index
        assert_eq!(router.current_history_index(), 2);
    }

    // assumes correct functioning of back() in the case of not currently at most recent history
    #[wasm_bindgen_test]
    fn test_forward() {
        let router: Router<ExampleRoutes> = Router::new();

        let pre_cond_idx = router.current_history_index();
        assert_eq!(pre_cond_idx, 0);
        let forward = router.forward();
        assert_eq!(forward, None, "We should Not have gone forwards");

        assert_eq!(
            router.current_history_index(),
            pre_cond_idx,
            "We should have current index 0"
        );

        router.navigate_to_new(ExampleRoutes::parse_path("").unwrap());
        router.navigate_to_new(ExampleRoutes::parse_path("register").unwrap());
        router.navigate_to_new(ExampleRoutes::parse_path("/dashboard/profile/55").unwrap());
        assert_eq!(router.current_history_index(), 2);

        let _ = router.back();
        let _ = router.back();

        assert_eq!(
            router.current_history_index(),
            0,
            "Sanity check we don't already meet a forward() pre-condition"
        );
        assert_ne!(
            forward,
            Some(ExampleRoutes::Register),
            "Sanity check we don't already meet a forward() pre-condition"
        );
        let forward = router.forward();
        assert_eq!(
            forward,
            Some(ExampleRoutes::Register),
            "We should have gone forward to \"register\""
        );
        assert_eq!(router.current_history_index(), 1);

        let forward = router.forward();
        assert_eq!(
            forward,
            Some(ExampleRoutes::Dashboard(DashboardRoutes::Profile(55))),
            "We should have gone forward to \"/dashboard/profile/55\""
        );
        assert_eq!(router.current_history_index(), 2);
        let forward = router.forward();
        assert_eq!(forward, None, "We should Not have gone forward");
        assert_eq!(
            router.current_history_index(),
            2,
            "History index should not change after empty-action forward()"
        );
    }
}
