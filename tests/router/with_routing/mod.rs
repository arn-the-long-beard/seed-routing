#[cfg(test)]
mod test {
    use crate::routing_module::pages;
    use crate::routing_module::pages::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use seed::{prelude::*, *};
    extern crate heck;
    use crate::routing_module::test::UserLogged;
    use seed_routing::{View, *};
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

        Model {
            dashboard: dashboard::Model::default(),
            user: None,
        }
    }

    #[derive(Debug, PartialEq, Clone, RoutingModules)]
    #[modules_path = "pages"]
    pub enum Route {
        #[view = "=> login"]
        Login,
        #[guard = "user => guard => forbidden"]
        Dashboard(dashboard::Route),
        #[default_route]
        #[view = "=> not_found"]
        NotFound,
        #[as_path = ""]
        #[view = "=> home"]
        Home,
    }

    // ------ ------
    //     Model
    // ------ ------
    struct Model {
        dashboard: dashboard::Model,
        user: Option<UserLogged>,
    }

    // ------ ------
    //    Update
    // ------ ------
    /// Root actions for your app.
    /// Each component will have single action/message mapped to its message later
    /// in update
    pub enum Msg {
        UrlChanged(subs::UrlChanged),
        Dashboard(dashboard::Msg),
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
            Msg::Dashboard(_) => {}
        }
    }
    pub fn forbidden(_: Option<&UserLogged>) -> Node<Msg> {
        div![]
    }
    pub fn guard(user: Option<&UserLogged>) -> Option<bool> {
        if user.is_some() {
            Some(true)
        } else {
            None
        }
    }
    // ------ ------
    //     View
    // ------ ------
    /// View function which renders stuff to html
    fn view(model: &Model) -> impl IntoNodes<Msg> {
        vec![header(&model), router().current_route().view(model)]
    }

    fn login(_: &Model) -> Node<Msg> {
        div!["login"]
    }

    fn home(_: &Model) -> Node<Msg> {
        div!["home"]
    }

    fn not_found(_: &Model) -> Node<Msg> {
        div!["not_found"]
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
        add_router!();
        let my_router = router();
        assert_eq!(my_router.current_route(), Route::NotFound);
        let url = Route::Login.to_url();
        router().init(url);
        assert_eq!(my_router.current_route(), Route::Login);
    }

    #[wasm_bindgen_test]
    fn test_router_and_route_view() {
        let my_router: Router<Route> = router();
        my_router.navigate_to_new(Route::Login);

        assert_eq!(
            my_router
                .current_route()
                .view(&Model {
                    dashboard: dashboard::Model::default(),
                    user: None,
                })
                .to_string(),
            login(&Model {
                dashboard: dashboard::Model::default(),
                user: None,
            })
            .to_string()
        );
    }
}
