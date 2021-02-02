use seed::{prelude::*, *};

extern crate router_derive;
extern crate seed_routing;

pub fn init(
    _: Url,
    model: &mut Model,
    query: &IndexMap<String, String>,
    _: &mut impl Orders<Msg>,
) -> Model {
    let id_query = query.get("admin_id");
    let privilege_query = query.get("admin_privilege");

    if let Some(id) = id_query {
        model.admin_id = id.to_string();
    }
    if let Some(p) = privilege_query {
        model.admin_privilege = p.to_string();
    }
    // it is ok to clone here because we just then replace the previous state with a new one
    model.clone()
}
pub enum Msg {}

#[derive(Default, Clone)]
pub struct Model {
    pub admin_id: String,
    pub admin_privilege: String,
}
pub fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {}
}
pub fn view(_: &Model) -> Node<Msg> {
    div![]
}
