//! This series of examples will demonstrate how seed-routing can be built, starting from a skeleton seed app.
//!
//! It starts with a seed app that provides a design sketch of the routing structure, with a theme change button hacked into it.
//! Because seed-routing builds on top of seed, make sure that you are comfortable with the "model, update, view" structure before
//! diving into the routing that we'll be adding.
//!
//! This series will be working towards recreating the backbone_app example. Before starting with this, run the backbone app
//! and play around with it for a minute or two.
//!

use seed::{prelude::*, *};
mod theme;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        theme: theme::Theme::Light,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    theme: theme::Theme,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ChangeTheme
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeTheme => {
            match model.theme {
                theme::Theme::Light => model.theme = theme::Theme::Dark,
                theme::Theme::Dark => model.theme = theme::Theme::Light,
            }
        }
    }
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

        button!["change theme", ev(Ev::Click, |_| Msg::ChangeTheme)]
    ]
}

fn header(model: &Model) -> Node<Msg> {
    div![

        // We'll be moving the theme-changing feature into one of our routes in later steps
        match model.theme {
            theme::Theme::Light => style! {St::Background => "lightskyblue", St::Color => "black"},
            theme::Theme::Dark => style! {St::Background => "blue", St::Color => "white"},
        },
        ["Welcome guest"],

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
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
