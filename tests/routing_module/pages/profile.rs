use super::*;

extern crate router_macro_derive;
extern crate seed_routing;
use router_macro_derive::*;
use seed::{prelude::*, *};

use wasm_bindgen_test::*;

pub fn init(url: Url, model: &mut Model, id: &String, orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}

pub struct Model {}
pub enum Msg {}
pub fn view(model: &Model) -> Node<Msg> {
    div![]
}

fn not_found(model: &Model) -> Node<Msg> {
    div![]
}

fn settings(model: &Model) -> Node<Msg> {
    div![]
}
