extern crate router_derive;
extern crate seed_routing;
use router_derive::*;
use seed_routing::*;

use seed::{prelude::*, *};

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
pub fn init(_: Url, _: &mut Model, _: &String, _: &Routes, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

pub struct Model {}

pub fn view(_: &Routes, _: &Model) -> Node<Msg> {
    div![]
}
pub enum Msg {}
pub fn root(_: &Model) -> Node<Msg> {
    div![]
}
pub fn video(_: &Model) -> Node<Msg> {
    div![]
}
pub fn files(_: &Model) -> Node<Msg> {
    div![]
}
