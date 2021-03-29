//! This series of examples will demonstrate how seed-routing can be built, starting from a skeleton seed app.
//!
//! step_3 showed us how to use the view attribute without passing in more data than is needed. So far, we have
//! limited ourselves to connecting simple `Route` enum variants to their views. Now, we will start using the
//! router to define some styling, and bring the in-app forward/back buttons to life.
//!
//! The real power of RoutingModules will be visited in the next step.
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
    // TODO (4) A seed app needs to be subscribed to url changes in order for the router to control the navigation.
    // Refer to seed-rs.org documentation covering the init function if you need a refresher.
    sync_router!();
    Model {
        theme: theme::Theme::Light,
    }
}

// At the moment, using RoutingModules requires these attributes
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {

    #[view = "theme => home"]
    Home,
    #[default_route]
    #[view = " => not_found"]
    NotFound,
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
    ChangeTheme,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeTheme => {
            match model.theme {
                theme::Theme::Dark => model.theme = theme::Theme::Light,
                theme::Theme::Light => model.theme = theme::Theme::Dark,
            }
        },
        // TODO (5) For changing url, we want to use the router. We can do this by getting
        // the routers current rout, and using the init function.


        // The third TODO needs to handle backwards and forwards.
        // the router does this with a call like this:
        // `router().request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));`
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

        // TODO (2) Let's bring these to life. Instead of having the `Disabled` attribute hard-coded as `true`,
        // we can ask the router. Get the router the same way as in TODO (1), and call the function that gives
        // forward/back information
        // TODO (3) Now, set the back button to trigger a `GoBack` message for the update function. The next
        // thing TODO, will be setting up the handling.
        div![
            style! {St::Display => "flex" },
            button![
                "back",
                attrs! {
                    At::Disabled  =>   (true /* the router can tell you if you're on the current route*/).as_at_value(),
                },
                // Add a click event here
            ],
            button![
                "forward",
                attrs! {
                    At::Disabled  =>   (true).as_at_value(),
                },
                // Add a click event here
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
        li![a![
            C![
                "route",
                // TODO! (1) Let's start by using the router to help us add some styling
                IF!(    router().is_current_route(&Routes::NotFound ) => "active-route" )
            ],
            attrs! { At::Href => &Routes::NotFound.to_url() },
            "not found"
        ]],
        li![a![
            C![
                "route",
                // TODO! (1) Let's start by using the router to help us add some styling
                IF!(    router().is_current_route(&Routes::Home ) => "active-route" )
            ],
            attrs! { At::Href => &Routes::Home.to_url() },
            "home"
        ]],
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

fn home(theme: &theme::Theme) -> Node<Msg> {
    div![
        button![
            format!{"change to {} theme", match theme {
                theme::Theme::Dark => "light",
                theme::Theme::Light => "dark",
            }},
            ev(Ev::Click, |_| Msg::ChangeTheme)
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
