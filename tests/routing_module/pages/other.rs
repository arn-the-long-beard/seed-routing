use super::*;
extern crate router_macro_derive;
extern crate seed_routing;
use router_macro_derive::*;
use seed_routing::*;

use seed::{prelude::*, *};
use wasm_bindgen_test::*;

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    #[default_route]
    #[view = " => root"]
    Root,
    #[view = " => files"]
    Files,
    #[view = " => video"]
    Video,
}
pub fn init(
    url: Url,
    model: &mut Model,
    id: &String,
    children: &Routes,
    orders: &mut impl Orders<Msg>,
) -> Model {
    Model {}
}

pub struct Model {}

pub fn view(children: &Routes, model: &Model) -> Node<Msg> {
    div![]
}
pub enum Msg {}
pub fn root(model: &Model) -> Node<Msg> {
    div![]
}
pub fn video(model: &Model) -> Node<Msg> {
    div![]
}
pub fn files(model: &Model) -> Node<Msg> {
    div![]
}
