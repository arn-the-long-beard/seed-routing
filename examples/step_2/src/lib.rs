//! This series of examples will demonstrate how seed-routing can be built, starting from a skeleton seed app.
//!
//! step_1 demonstrated a stripped-down version of the backbone app. Now, it's time to start bringing in seed-routing.
//! We'll start by giving the router a default route to fall back to if it's asked to go somewhere that doesn't exist.
//! In other words, a 404-not found.
//!
//! To kick off, there are two TODO's. When completed, you can start the familiar cycle of compile - fix error - repeat
//! All the fixes are already there, but commented out. Resolve the errors one by one, from the top down. Make sure you
//! understand

use seed::{prelude::*, *};
#[macro_use]
extern crate seed_routing;
use seed_routing::{View, *};
mod theme;

// ------ ------
//     Init
// ------ ------

// TODO: uncomment
// add_router!();

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // TODO: uncomment
    // sync_router!();
    Model {
    }
}

// At the moment, using RoutingModules requires these attributes as well as a default implementation.
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    // The compiler will complain that Default isn't implemented. The #[default_route] attribute implements
    // this under the hood, giving the router a fall-back route. By convention, a 404 page fits well.
    // #[default_route]


    // The compiler will complain that we don't have a crate or module `not_found`
    // For each route, the router needs a view in order to render it. Because this view is simple, we're not so
    // interested in putting it into a dedicated module for now, so we'll just keep it here in the root module.
    // So we need to direct the compiler to the function in this module used to render the `NotFound` view.
    //
    // #[view = " => not_found"]
    NotFound,
}

// ------ ------
//     Model
// ------ ------

struct Model {
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
}

// ------ ------
//     View
// ------ ------
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        // We'll be using seed-routing to bring the back and forward button to life
        header(&model),

        // These links are hard-coded for now. We will be using `#derive(RoutingModules)` to bring them to life.
        links(),

        div!["We've removed the change theme button for now."],
        div!["======================================================"],
        div!["Once you uncomment `router().current_route().view(model)`, everything below this is a view coming from the router"],
        div!["======================================================"],

        // router().current_route().view(model)

    ]
}

fn header(model: &Model) -> Node<Msg> {
    div![

        // We'll be moving the theme-changing feature into one of our routes in later steps
        style! {St::Background => "lightskyblue"},
        "Welcome guest",

        // Ghosted routing buttons that will be brought to life in later steps
        div![
            style! {St::Display => "flex" },
            button![
                "back",
                attrs! {
                    At::Disabled  =>   (true).as_at_value(),
                },
            ],
            button![
                "forward",
                attrs! {
                    At::Disabled =>  (true).as_at_value(),
                },
            ],
            span![style! {St::Flex => "5" },],
            span![button![
                attrs! {
                    At::Disabled  =>   (true).as_at_value(),
                },
                "sign in ",
                C!["user_button"],
                i![C!["fas fa-user-circle"]]
            ]]
        ],
    ]
}

// These will become links to different routes.
fn links() -> Node<Msg> {
    ul![
        li!["login"],
        li!["login for JohnDoe"],
        li!["Not Found"],
        li!["Home"],
        li!["Admin"],
        ul![
            li!["project 1"],
            li!["project 2"],
            li!["project 3"],
            li!["Not found project 3"],
            li!["Manage project 1"],
        ],
        li!["Dashboard"],
        ul![
            li!["profile"],
            li!["Message"],
            li!["Statistics"],
            li!["Tasks"],
            li!["Tasks with url query"],
        ]
    ]
}
// The `#[view = " => not_found"]` attribute will tell the router that when it wants to render for
// the `NotFound` route, this is the function to use. This should be the last thing to uncomment
// to clear up all the errors, but there's still one more needed before the router can put the
// view onto display. Serve the website, and you'll see the instructions on what to do then.
//
// fn not_found(model: &Model) -> Node<Msg> {
//     div!["404 - page not found"]
// }
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
