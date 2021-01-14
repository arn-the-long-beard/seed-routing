#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use seed::{prelude::*, *};
    extern crate heck;
    use seed_routing::*;
    use std::fmt::Debug;
    add_router!();
    // ------ ------
    //     Init
    // ------ ------
    fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders.subscribe(Msg::UrlChanged);

        router().init(url).subscribe(orders.subscribe_with_handle(
            |subs::UrlRequested(requested_url, _)| router().confirm_navigation(requested_url),
        ));

        Model {}
    }
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

    #[derive(Debug, PartialEq, Clone, ParseUrl, WithDefaultRoute)]
    pub enum Route {
        Login,
        Register,
        Stuff,
        Dashboard(DashboardRoutes),
        #[default_route]
        NotFound,
        #[as_path = ""]
        Home,
    }

    // ------ ------
    //     Model
    // ------ ------
    struct Model {}

    // ------ ------
    //    Update
    // ------ ------
    /// Root actions for your app.
    /// Each component will have single action/message mapped to its message later
    /// in update

    pub enum Msg {
        UrlChanged(subs::UrlChanged),
        GoBack,
        GoForward,
    }

    fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::UrlChanged(subs::UrlChanged(_url)) => {}
            Msg::GoBack => {
                router().request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));
            }
            Msg::GoForward => {
                router().request_moving_forward(|r| orders.notify(subs::UrlRequested::new(r)));
            }
        }
    }

    // ------ ------
    //     View
    // ------ ------
    /// View function which renders stuff to html
    fn view(model: &Model) -> impl IntoNodes<Msg> {
        vec![header(&model)]
    }

    fn header(model: &Model) -> Node<Msg> {
        div![]
    }
    // ------ ------
    //     Start
    // ------ ------

    // #[wasm_bindgen(start)]
    // pub fn start() {
    //     App::start("app", init, update, view);
    // }

    #[wasm_bindgen_test]
    fn test_router_init() {
        let my_router = router();
        assert_eq!(my_router.current_route(), Route::NotFound);

        let url = Route::Login.to_url();
        router().init(url);
        assert_eq!(my_router.current_route(), Route::Login);
    }
}
