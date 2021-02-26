use seed::{prelude::*, *};

pub mod pages;

#[cfg(test)]
pub mod test {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;
    extern crate router_derive;
    extern crate seed_routing;
    use crate::routing_module::pages::{admin, dashboard, other, profile};
    use router_derive::*;

    use seed_routing::{View, *};

    pub struct UserLogged {
        pub name: String,
    }

    pub struct Model {
        user: Option<UserLogged>,
        other: other::Model,
        dashboard: dashboard::Model,
        admin: admin::Model,
        profile: profile::Model,
    }

    pub enum Msg {
        Other(other::Msg),
        Dashboard(dashboard::Msg),
        Admin(admin::Msg),
        Profile(profile::Msg),
    }

    #[derive(Debug, PartialEq, Clone, RoutingModules)]
    #[modules_path = "pages"]
    pub enum SuperExampleRoutes {
        Other {
            id: String,
            children: pages::other::Routes,
        },
        Admin {
            query: IndexMap<String, String>,
        },
        #[guard = "user => guard => forbidden"]
        Dashboard(dashboard::Routes),
        Profile {
            id: String,
        },
        #[default_route]
        #[view = " => not_found"]
        NotFound,
        #[as_path = ""]
        #[view = " => home"]
        Root,
    }

    pub fn _init() {}

    pub fn _view(_: &Model) -> Node<Msg> {
        div!["my_view"]
    }

    pub fn home(_: &Model) -> Node<Msg> {
        div!["home"]
    }

    pub fn _update() {}

    pub fn not_found(_: &Model) -> Node<Msg> {
        div!["not_found"]
    }
    pub fn forbidden(user: Option<&UserLogged>) -> Node<Msg> {
        if user.is_some() {
            div![format!("forbidden for you {}", user.unwrap().name)]
        } else {
            div!["forbidden"]
        }
    }
    pub fn guard(user: Option<&UserLogged>) -> Option<bool> {
        if let Some(user_logged) = user {
            if user_logged.name == "tester" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    }

    #[wasm_bindgen_test]
    fn test_guard() {
        assert_eq!(guard(None), None);
        assert_eq!(
            guard(Some(&UserLogged {
                name: "normal_user".to_string(),
            })),
            Some(false)
        );
        assert_eq!(
            guard(Some(&UserLogged {
                name: "tester".to_string(),
            })),
            Some(true)
        );
    }

    #[wasm_bindgen_test]
    fn test_view() {
        let view_from_not_found = SuperExampleRoutes::NotFound
            .view(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                user: None,
                other: other::Model::default(),
                profile: profile::Model {},
            })
            .to_string();

        assert_eq!(
            view_from_not_found,
            not_found(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                user: None,
                other: other::Model::default(),
                profile: profile::Model {},
            })
            .to_string()
        );

        let view_from_home = SuperExampleRoutes::Root
            .view(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                user: None,
                other: other::Model::default(),
                profile: profile::Model {},
            })
            .to_string();

        assert_eq!(
            view_from_home,
            home(&Model {
                dashboard: dashboard::Model::default(),
                admin: admin::Model::default(),
                user: None,
                other: other::Model::default(),
                profile: profile::Model {},
            })
            .to_string()
        );
    }

    #[wasm_bindgen_test]
    fn test_view_with_guard_without_logged_user() {
        let view_from_route_without_logged_user =
            SuperExampleRoutes::Dashboard(dashboard::Routes::Settings)
                .view(&Model {
                    dashboard: dashboard::Model::default(),
                    admin: admin::Model::default(),
                    user: None,
                    other: other::Model::default(),
                    profile: profile::Model {},
                })
                .to_string();

        assert_eq!(
            view_from_route_without_logged_user,
            forbidden(None).to_string()
        );
    }

    #[wasm_bindgen_test]
    fn test_view_with_guard_with_logged_user() {
        let view_from_route_with_logged_user =
            SuperExampleRoutes::Dashboard(dashboard::Routes::Settings)
                .view(&Model {
                    dashboard: dashboard::Model::default(),
                    admin: admin::Model::default(),
                    user: Some(UserLogged {
                        name: "tester".to_string(),
                    }),
                    other: other::Model::default(),
                    profile: profile::Model {},
                })
                .to_string();

        assert_eq!(
            view_from_route_with_logged_user,
            dashboard::settings(&dashboard::Model::default()).to_string()
        );

        let view_from_route_with_wrong_logged_user =
            SuperExampleRoutes::Dashboard(dashboard::Routes::Settings)
                .view(&Model {
                    dashboard: dashboard::Model::default(),
                    admin: admin::Model::default(),
                    user: Some(UserLogged {
                        name: "normal_user".to_string(),
                    }),
                    other: other::Model::default(),
                    profile: profile::Model {},
                })
                .to_string();

        assert_eq!(
            view_from_route_with_wrong_logged_user,
            forbidden(Some(&UserLogged {
                name: "normal_user".to_string()
            }))
            .to_string()
        );
    }
}
