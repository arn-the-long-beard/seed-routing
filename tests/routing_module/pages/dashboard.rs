extern crate router_derive;
extern crate seed_routing;
use router_derive::*;
use seed_routing::*;

use seed::{prelude::*, *};

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    #[default_route]
    #[view = " => not_found"]
    NotFound,
    #[view = " => settings"]
    Settings,
}
pub enum Msg {}
pub fn init(_: Url, _: &mut Model, _: &Routes, _: &mut impl Orders<Msg>) -> Model {
    Model {
        stuff: "init_has_been_done".to_string(),
    }
}
#[derive(Default)]
pub struct Model {
    pub stuff: String,
}
pub fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {}
}

pub fn view(route: &Routes, model: &Model) -> Node<Msg> {
    route.view(model)
}

fn not_found(_: &Model) -> Node<Msg> {
    div!["not_found"]
}

pub fn settings(_: &Model) -> Node<Msg> {
    div!["settings"]
}
