use seed::{prelude::*, *};

pub mod pages;

#[cfg(test)]
pub mod test {

    use super::*;
    extern crate router_macro_derive;
    extern crate seed_routing;
    use crate::routing_module::pages::{admin, dashboard, other, profile};
    use router_macro_derive::*;
    use seed::{prelude::*, *};
    use seed_routing::*;

    use wasm_bindgen_test::*;

    pub struct UserLogged {
        name: String,
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

    pub fn init() {}

    pub fn view(model: &Model) -> Node<Msg> {
        div![]
    }

    pub fn home(model: &Model) -> Node<Msg> {
        div![]
    }

    pub fn update() {}

    pub fn not_found(model: &Model) -> Node<Msg> {
        div![]
    }
    pub fn forbidden(user: Option<&UserLogged>) -> Node<Msg> {
        div![]
    }
    pub fn guard(user: Option<&UserLogged>) -> Option<bool> {
        if user.is_some() {
            Some(true)
        } else {
            None
        }
    }
    // pub fn guard(model : &Model) -> Option<bool> {
    //     if  model.user.is_some() {
    //         Some(true)
    //     } else {
    //         None
    //     }
    // }
}
