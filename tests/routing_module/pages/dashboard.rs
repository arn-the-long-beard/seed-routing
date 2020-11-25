extern crate router_macro_derive;
extern crate seed_routing;
use router_macro_derive::*;
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
pub fn init(_: Url, _: &mut Model, _: &Routes, _: &mut impl Orders<Msg,>,) -> Model {
    Model { _stuff: "".to_string(), }
}

pub struct Model {
    _stuff: String,
}

pub fn view(_: &Routes, _: &Model,) -> Node<Msg,> {
    div![]
}

fn not_found(_: &Model,) -> Node<Msg,> {
    div![]
}

fn settings(_: &Model,) -> Node<Msg,> {
    div![]
}
