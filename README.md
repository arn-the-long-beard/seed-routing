## Seed Routing

A proposal for advanced routing in Seed equivalent to what we have in Angular/React/Vue.


### Summary

- [Quickest way to use it](#quickest-way-to-use-it)
- [Rules](#rules)
- [Motivation](#motivations)
- [Description](#description)
- [Example ParseUrl](#example-code-with-parseurl)
- [Example DefaultRoute](#example-code-with-defaultroute)
- [Example Modules](#example-code-with-routingmodules)

### Quickest way to use it

#### 1 - Add `seed_routing` to your project in Cargo.toml:

```toml

seed_routing = { git="https://github.com/arn-the-long-beard/seed-routing.git" ,branch ="main" }

```
#### 2 - Implement the router in lib.rs

In your lib.rs you need this code :

- with init : 

```rust

add_router!();  

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    sync_router!();

    Model {
        // Your Model
    }
}

```

- with update :


```rust

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(_url)) => {
            router().current_route().init(model, orders);
        }
        // rest of the match arms.
    }
}
```

- with view :

```rust

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![router().current_route().view(model)]
}

```

There is nothing more to write for the router to do its job.

#### 3 - Write your `Routes` enum starting in lib.rs :

```rust
#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Login {
        query: IndexMap<String, String>, // -> http://localhost:8000/login?name=JohnDoe
    },
    #[guard = " => guard => forbidden"]
    Dashboard(pages::dashboard::Routes), // -> http://localhost:8000/dashboard/*
    #[guard = "logged_user => admin_guard => forbidden_user"]
    Admin {
        // -> /admin/:id/*
        id: String,
        children: pages::admin::Routes,
    },
    #[default_route]
    // 
    #[view = " => not_found"]            // -> http://localhost:8000/not_found*
    NotFound,
    #[view = " => forbidden"]            // -> http://localhost:8000/forbidden*
    Forbidden,
    #[as_path = ""]
    #[view = " => home"]                 // -> http://localhost:8000/
    Home,
}

```
#### 4 - Use the alpha cli for generating files & code from `Routes` enum :

`cargo install proto_seeder`

In the route of your project :

`proto_seeder ./src/lib.rs`


This will generate from `Routes` for you the following :

- local views.
- guards.
- sub modules with their content.
- init with payload.
- empty Model.
- empty Message.
- update.
- view.

Here is the experimental cli repo : https://github.com/arn-the-long-beard/proto-seeder

It is still very alpha and it needs more inputs & feedbacks to get better !

#### 5 - You can write a new `Routes` enum in a submodule and rerun the command from `proto_seeder`.

`proto_seeder ./src/dashboard.rs`

It will generate the submodules/codes for it as well.

Enjoy the saving time !


### Rules

Here are the rules I try to respect
- Respect for TEA = One single path of truth.
- Minimize the boilerplate and the risks of bugs.
- Easy API so we can follow the flow of an app.
- Making routing in TEA should be as easy as in popular frameworks.

### Motivations

My idea and motivation come mainly from the fact that I had a very bad experience with large App in Elm.

I got pain there with :
- Routing was absent.
- Single path of truth is hard to see.
- How to add and load components was not framed.
- Need to update 5 or 6 files code in order to just add a tiny component.
- Could not see guards and protected routes that require user permission or authentication.

I also got a lot of happiness doing routing in Angular
- Routes are easy to write with the path, guard and component or module to load.
- Sub routes are easy since you add them to sub modules and they are added automatically later on.
- Lazy loading is easy.
- Guarding routes is easy.
- You can pass data to routes.
- The router is easy to use via dependency injection at runtime.
- RouterLink is OP in html template.
```html
<a [routerLink]="['/user/bob']" [queryParams]="{debug: true}" queryParamsHandling="merge">
  link to user component
</a>
```
- Management state from the router is possible with Ngrx.
- Little code in html.

```html
    <router-outlet></router-outlet>
```
- Routing is framed as a standard in Angular so you know where to find consistent information to make good routing.


The discussion regarding routing in seed is there https://github.com/seed-rs/seed/issues/383

### Description



This repos actually contains 2 distinct but linked concepts :

- A **router** for : 
    - Managing history.
    - Go back.
    - Go forward .
    - Use default route.
    - Standard navigation.
    - You know if you can go back of forward and display it on UI easy.


- Derive macro for generating Url and call to init & view with guard functions

    - With **ParseUrl** , you can easily convert URL for all your enums and vis versa.
    - With **RoutingModules**, you will get :
        - Nested route / children routes.
        - Id parameter / dynamic urls.
        - Search query / query parameters.
        - Guard with protected routes :
            - Inject the model you need.
            - Give you a view to display when access is deny.
            - Will display the right view in the router.view().
        - init function calls & match automatically ( not protected but we could to it => Todo) :
            - Has access to URL payload ( query, id & children) .
            - Let you init your model from the previous state.
            - Let you init your model based on Url payload ( Awesome :D ).
        - view function calls & Match.
            - Has access to children & nested .
            - Is accessible only if condition are true from the guard.
       



### Example code with ParseUrl

 Derive an enum to enable conversion `from` and to `url`.
 You can change the value of a path for a given route this way.
 ```rust
 #[derive(Debug, PartialEq, Copy, Clone, ParseUrl)]
 pub enum DashboardAdminRoutes {
     // "/my_stuff"
     #[as_path = "my_stuff"]
     Other,

     // ""
     #[as_path = ""]
     Root,
 }

 fn test_url() {
     let mut query_search: IndexMap<String, String> = IndexMap::new();

     query_search.insert("user".to_string(), "arn".to_string());
     query_search.insert("role".to_string(), "baby_programmer".to_string());
     query_search.insert("location".to_string(), "norway".to_string());
     let url = ExampleRoutes::Admin {
         query: query_search.clone(),
     }
     .to_url();
     let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
         .parse()
         .unwrap();
     assert_eq!(url, url_to_compare);
 }
 ```
### Example code with DefaultRoute

 Define a routing config with a default_route for your navigation.
 The default route used by the router when it cannot find the
 right url.

 ```rust
 #[derive(Debug, PartialEq, Copy, Clone, DefaultRoute)]
 pub enum DashboardAdminRoutes {
     // An invalid route, such as /blablablabla, re-routes to /not_found
     #[default_route]
     NotFound,
     Root,
 }
 ```
### Example code with RoutingModules

RoutingModule contains DefaultRoute and ParseUrl as well.

 ```rust
use seed::{prelude::*, *};
#[macro_use]
extern crate seed_routing;
use seed_routing::{View, *};
add_router!();

enum Theme {
    Dark,
    Light
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

 fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
     orders.subscribe(Msg::UrlChanged).subscribe(Msg::UserLogged);

     router().init(url).subscribe(orders.subscribe_with_handle(
         |subs::UrlRequested(requested_url, _)| router().confirm_navigation(requested_url),
     )); // ---> is equal to sync_router!();
     
     Model {
         register: Default::default(),
         login: Default::default(),
         dashboard: Default::default(),
         admin: Default::default(),
         router,
         logged_user: None,
         theme: Theme::default(),
     }
 }

 struct Model {
     pub register: pages::register::Model,
     pub login: pages::login::Model,
     pub dashboard: pages::dashboard::Model,
     pub admin: pages::admin::Model,
     router: Router<Routes>,
     logged_user: Option<LoggedUser>,
     theme: Theme,
 }

 #[derive(Debug, PartialEq, Clone, RoutingModules)]
 pub enum Routes {
     // http://localhost:8000/register
     Register,
     
     // will handle: 
     // * loading "admin.rs" file
     // * loading model.admin field
     // * verify init function arguments
     // * verify view function arguments
     // These are done for other Routes variants assosciated with a module.
     #[guard = "logged_user => admin_guard => forbidden_user"]
     Admin { 
         query: IndexMap<String, String>,
     },

     // http://localhost:8000/login?name=JohnDoe
     // query pairs are held and generated by this query field
     Login {
         query: IndexMap<String, String>,
     },
     
     #[guard = " => user_guard => forbidden"]
     Dashboard(DashboardRoutes),
     Profile { // will load module "profile"
         id: String,
     },
     
     // use the value of the `logged_user` field in the `admin_guard` function, 
     // falling back to the view generated by `fn forbidden_user` in case of
     // `admin_guard` not returning `Some(true)`
     #[guard = "logged_user => admin_guard => forbidden_user"]
     #[view = " => my_stuff"]
     MyStuff,

     // use the value of of the `theme` field in the model in the view generating
     // function `theme_eg` when creating the view for the `/theme_example` route
     #[view = "theme => theme_eg"]
     ThemeExample,
     #[view = " => not_found"]

     #[default_route]
     NotFound,

     #[view = " => home"]
     #[as_path = ""]
     Root,
 }

 fn user_guard(model: &Model) -> Option<bool> {
     // could check local storage, cookie or what ever you want
     if model.logged_user.is_some() {
         Some(true)
     } else {
         None
     }
 }
 fn admin_guard(logged_user: Option<&LoggedUser>) -> Option<bool> {
     // could check local storage, cookie or what ever you want
     if let Some(user) = logged_user {
         match user.role {
             Role::StandardUser => Some(false),
             Role::Admin => Some(true),
         }
     } else {
         None
     }
 }
 
 fn not_found(_: &Model) -> Node<Msg> {
     div!["404 page not found"]
 }
 
 fn forbidden(_: &Model) -> Node<Msg> {
     div!["401 access denied"]
 }
 
 fn forbidden_user(logged_user: Option<&LoggedData>) -> Node<Msg> {
     if let Some(user) = logged_user {
         p![format!(
             "Sorry {} {} , but you are missing the Admin Role. Ask your administrator for more information. ",
             user.first_name, user.last_name
         )]
     } else {
         div!["401"]
     }
 }

 fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
     match msg {
         Msg::UrlChanged(subs::UrlChanged(_)) => {
             router().current_route().init(model, orders);
         }
         // ...remaining arms
     }
 }


 fn view(model: &Model) -> impl IntoNodes<Msg> {
     vec![
         header(&model),
         router().current_route().view(model)
     ]
 }
 
 fn theme_eg(theme: &Theme) -> impl IntoNodes<Msg> {
    match theme {
        Theme::Dark => {/* ... */},
        Theme::Light => {/* ... */},
    }
 }

 ```


### Router life cycle

#### The router listens to Url changes

![Entity Diagram](assets/router_listening.svg)

#### You can request to move back

![Entity Diagram](assets/router_go_back.svg)

Then we go back to the router listening

#### You can request to move forward

![Entity Diagram](assets/router_go_forward.svg)

Then we go back to the router listening


### Use the example
```bash
cd examples/backbone_app
```



```bash
cargo make watch 

cargo make serve
```

Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

### Author

[arn-the-long-beard](https://github.com/arn-the-long-beard)

### Contributors

[Ben-PH](https://github.com/Ben-PH)

[mankinskin](https://github.com/mankinskin)

### Sources

https://github.com/mankinskin/enum_paths

https://github.com/TatriX/seed-router
