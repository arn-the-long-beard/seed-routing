use seed::{prelude::*, *};

pub mod pages;

#[cfg(test)]
pub mod test {

    use super::*;
    extern crate router_macro_derive;
    extern crate seed_routing;
    use crate::routing_module::pages::{admin, dashboard, other, profile};
    use router_macro_derive::*;

    use seed_routing::*;

    pub struct UserLogged {
        _name: String,
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
        div![]
    }

    pub fn home(_: &Model) -> Node<Msg> {
        div![]
    }

    pub fn _update() {}

    pub fn not_found(_: &Model) -> Node<Msg> {
        div![]
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
}
