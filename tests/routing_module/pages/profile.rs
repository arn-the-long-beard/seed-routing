extern crate router_macro_derive;
extern crate seed_routing;
use seed::{prelude::*, *};
pub fn init(_: Url, _: &mut Model, _: &String, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

pub struct Model {}
pub enum Msg {}
pub fn view(_: &Model) -> Node<Msg> {
    div![]
}
