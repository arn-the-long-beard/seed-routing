extern crate router_macro_derive;
extern crate seed_routing;
use router_macro_derive::*;
use seed_routing::*;

use seed::{prelude::*, *};

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Route {
    #[default_route]
    #[view = " => not_found"]
    NotFound,
    #[view = " => settings"]
    Settings,
}
pub enum Msg {}
pub fn init(_: Url, _: &mut Model, _: &Route, _: &mut impl Orders<Msg>) -> Model {
    Model {
        _stuff: "".to_string(),
    }
}
#[derive(Default)]
pub struct Model {
    _stuff: String,
}
pub fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {}
}

pub fn view(route: &Route, model: &Model) -> Node<Msg> {
    route.view(model)
}

fn not_found(_: &Model) -> Node<Msg> {
    div!["not_found"]
}

pub fn settings(_: &Model) -> Node<Msg> {
    div!["settings"]
}
