#[cfg(test)]
mod test {
    use crate::routing_module::pages;
    use crate::routing_module::pages::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use seed::{prelude::*, *};
    extern crate heck;
    use crate::router::with_routing::test::Route::Dashboard;
    use crate::routing_module::test::UserLogged;
    use seed::app::OrdersContainer;
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
            admin: admin::Model::default(),
            other: other::Model::default(),
            user: None,
        }
    }
    /// The base routes of the application.
    /// If a submodule to load from a route has also routes, they should appear as children routes as well.
    #[derive(Debug, PartialEq, Clone, RoutingModules)]
    #[modules_path = "pages"]
    pub enum Route {
        #[view = "=> login"]
        Login,
        #[guard = "user => guard => forbidden"]
        Dashboard(dashboard::Route),
        Admin {
            query: IndexMap<String, String>,
        },
        #[as_path = "my_stuff"]
        Other {
            id: String,
            children: other::Routes,
        },
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
        admin: admin::Model,
        other: other::Model,
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
        Admin(admin::Msg),
        Other(other::Msg),
        GoBack,
        GoForward,
    }

    /// The standard update in a seed app.
    fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::UrlChanged(subs::UrlChanged(_url)) => {
                router().current_route().init(model, orders);
            }

            Msg::GoBack => {
                router().request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));
            }
            Msg::GoForward => {
                router().request_moving_forward(|r| orders.notify(subs::UrlRequested::new(r)));
            }
            Msg::Dashboard(dashboard_message) => dashboard::update(
                dashboard_message,
                &mut model.dashboard,
                &mut orders.proxy(Msg::Dashboard),
            ),
            Msg::Admin(admin_message) => admin::update(
                admin_message,
                &mut model.admin,
                &mut orders.proxy(Msg::Admin),
            ),
            Msg::Other(other_message) => other::update(
                other_message,
                &mut model.other,
                &mut orders.proxy(Msg::Other),
            ),
        }
    }
    pub fn forbidden(_: Option<&UserLogged>) -> Node<Msg> {
        div!["401"]
    }
    pub fn guard(user: Option<&UserLogged>) -> Option<bool> {
        if let Some(user_logged) = user {
            if user_logged.name.is_empty() {
                Some(false)
            } else {
                Some(true)
            }
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

    /// A local view login
    fn login(_: &Model) -> Node<Msg> {
        div!["login"]
    }

    /// A local view home
    fn home(_: &Model) -> Node<Msg> {
        div!["home"]
    }

    /// A local view for not found
    fn not_found(_: &Model) -> Node<Msg> {
        div!["not_found"]
    }

    /// A header view
    fn header(model: &Model) -> Node<Msg> {
        div!["header"]
    }
    // ------ ------
    //     Start
    // ------ ------

    // #[wasm_bindgen(start)]
    // pub fn start() {
    //   ;
    // }

    // pub fn get_app() -> App<Msg, Model, Node<Msg>> {
    //
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

        let current_view = my_router
            .current_route()
            .view(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                other: other::Model::default(),
                user: None,
            })
            .to_string();
        let login_view = login(&Model {
            dashboard: dashboard::Model::default(),
            admin: admin::Model::default(),
            other: other::Model::default(),
            user: None,
        })
        .to_string();

        assert_eq!(current_view, login_view);
    }

    #[wasm_bindgen_test]
    fn test_router_and_route_view_with_children() {
        let my_router: Router<Route> = router();
        my_router.navigate_to_new(Route::Other {
            id: "123".to_string(),
            children: other::Routes::Root,
        });

        let current_view = my_router
            .current_route()
            .view(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                other: other::Model::default(),
                user: None,
            })
            .to_string();

        let other_view = other::root(&other::Model::default()).to_string();

        assert_eq!(current_view, other_view);
    }

    #[wasm_bindgen_test]
    fn test_router_view_and_guard() {
        let my_router: Router<Route> = router();

        my_router.navigate_to_new(Dashboard(dashboard::Route::Settings));
        let current_view = my_router
            .current_route()
            .view(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                other: other::Model::default(),
                user: None,
            })
            .to_string();
        assert_eq!(
            current_view,
            forbidden(Some(&UserLogged {
                name: "".to_string()
            }))
            .to_string()
        );

        let test_user = UserLogged {
            name: "test_user".to_string(),
        };
        my_router.navigate_to_new(Dashboard(dashboard::Route::Settings));

        let current_view = my_router
            .current_route()
            .view(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                other: other::Model::default(),
                user: Some(test_user),
            })
            .to_string();

        assert_eq!(
            current_view,
            dashboard::settings(&dashboard::Model::default()).to_string()
        );
    }

    #[wasm_bindgen_test]
    fn test_router_navigation_and_page_init() {
        let mut model = Model {
            dashboard: dashboard::Model::default(),
            admin: admin::Model::default(),
            other: other::Model::default(),
            user: None,
        };
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let val = document
            .create_element("section")
            .expect("should add section");
        val.set_id("app");

        let app = App::start(val, init, update, view);
        let mut orders = OrdersContainer::new(app);
        let url = Route::Home.to_url(); // Home is chosen by wasm_pak probably because it does query the base url first from Seed
        update(
            Msg::UrlChanged(subs::UrlChanged(url)),
            &mut model,
            &mut orders,
        );

        assert_eq!(&model.dashboard.stuff, "");
        let dashboard_url = Route::Dashboard(dashboard::Route::Settings).to_url();
        router().navigate_to_url(dashboard_url.clone());

        update(
            Msg::UrlChanged(subs::UrlChanged(dashboard_url)),
            &mut model,
            &mut orders,
        );

        assert_eq!(&model.dashboard.stuff, "init_has_been_done");

        let admin_url: Url = "http://localhost/admin?admin_id=1&admin_privilege=high"
            .parse()
            .unwrap();

        router().navigate_to_url(admin_url.clone());

        update(
            Msg::UrlChanged(subs::UrlChanged(admin_url)),
            &mut model,
            &mut orders,
        );

        assert_eq!(&model.admin.admin_id, "1");
        assert_eq!(&model.admin.admin_privilege, "high");
    }
    #[wasm_bindgen_test]
    fn test_router_navigation_and_page_init_with_id_and_rename() {
        let mut model = Model {
            dashboard: dashboard::Model::default(),
            admin: admin::Model::default(),
            other: other::Model::default(),
            user: None,
        };
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let val = document
            .create_element("section")
            .expect("should add section");
        val.set_id("app");

        let app = App::start(val, init, update, view);
        let mut orders = OrdersContainer::new(app);

        let stuff_url: Url = "http://localhost/my_stuff/123/files".parse().unwrap();
        router().navigate_to_url(stuff_url.clone());
        update(
            Msg::UrlChanged(subs::UrlChanged(stuff_url)),
            &mut model,
            &mut orders,
        );

        let current_view = router().current_route().view(&model).to_string();
        assert_eq!(
            current_view,
            other::files(&other::Model {
                id: "123".to_string()
            })
            .to_string()
        );
    }
}
