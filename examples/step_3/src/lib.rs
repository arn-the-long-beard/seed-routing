//! This series of examples will demonstrate how seed-routing can be built, starting from a skeleton seed app.
//!
//! step_2 was the first look at a seed-routing feature: Having a `Route` enum with a default route. We will be
//! following up on this by bringing in the home route and the theme change button. The view function won't pass
//! in the entire model, but rather, only the data needed.
//!
//! In the next step, we will be looking at adding the hyper links, and activating the back/forward buttons.
//!
//! This demonstration works with a series of TODO's start with TODO (1), and work through to TODO (n)

use seed::{prelude::*, *};
#[macro_use]
extern crate seed_routing;
use seed_routing::{View, *};
mod theme;

// ------ ------
//     Init
// ------ ------

add_router!();

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    sync_router!();
    Model {
        // TODO (2): We don't want to hard-code the theme any more. The theme rendered should be based
        // on the data inside the model.
        // theme: theme::Theme::Light,
    }
}

// At the moment, using RoutingModules requires these attributes as well as a default implementation.
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {

    // TODO (1): The router needs to go to a view function that contains the theme-change button. Use what you
    // learned in step 2 to tell the router which function to use when at `/home`. For now, define a function that just renders a de-activated
    // button labelled "Change Theme". You can copy some code from the header function for the button.
    // TODO (3): You should only need one field from the `Model` to define how the home page is rendered.
    // Instead of giving the view function the entire model, we can single out a field within the model.
    // If the function name is `home`, and the model contains a field `theme: theme::Theme`, then the view
    // attribute would be `#[view = "theme => home"]`
    #[view = " => "]
    Home,
    #[default_route]
    #[view = " => not_found"]
    NotFound,
}

// ------ ------
//     Model
// ------ ------

struct Model {
    // theme: theme::Theme,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    // ChangeTheme,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    // TODO (4): At this point you should have a view function that takes in one of the models fields as an argument, and shows a deactivated button.
    // This view is shown at the bottom of the page when navigating to `/home`
    //
    // At this point, we use the seed structure to change the theme. Uncomment the code below, and rewrite button code in the home page view function
    // to trigger the theme change.
    //
    // match msg {
    //     Msg::ChangeTheme => {
    //         match model.theme {
    //             theme::Theme::Dark => model.theme = theme::Theme::Light,
    //             theme::Theme::Light => model.theme = theme::Theme::Dark,
    //         }
    //     }
    // }
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
        div!["everything below this is a view coming from the router"],
        div!["======================================================"],

        router().current_route().view(model)

    ]
}

fn header(model: &Model) -> Node<Msg> {
    div![

        match model.theme {
            theme::Theme::Light => style! {St::Background => "lightskyblue", St::Color => "black"},
            theme::Theme::Dark => style! {St::Background => "blue", St::Color => "white"},
        },

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
fn not_found(model: &Model) -> Node<Msg> {
    div!["404 - page not found"]
}


// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
