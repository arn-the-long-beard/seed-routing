use seed::{prelude::*, *};

extern crate router_macro_derive;
extern crate seed_routing;

pub fn init(_: Url, _: &mut Model, _: &IndexMap<String, String,>, _: &mut impl Orders<Msg,>,) -> Model {
    Model {}
}
pub enum Msg {}
pub struct Model {}

pub fn view(_: &Model,) -> Node<Msg,> {
    div![]
}
