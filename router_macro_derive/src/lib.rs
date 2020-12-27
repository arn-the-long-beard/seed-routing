#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate convert_case;
extern crate proc_macro;
extern crate proc_macro_error;

use crate::{root::get_default_route, routing::routing_variant_snippets};
use proc_macro::TokenStream;

use crate::{init::module_init_snippets, view::modules_view_snippets};
use proc_macro_error::{abort, proc_macro_error, Diagnostic, Level};
use quote::quote;
use syn::{export::TokenStream2, parse_macro_input, Data, DeriveInput, Fields};

mod builder;
mod guard;
mod init;
mod modules;
mod root;
mod routing;
mod view;

/// Add the router to the web application
///
/// ```rust
/// add_router!();
///
///
/// enum Route {
///     Home,
///     NotFound
/// }
///
/// router().set_current_route(Route::Home);
/// assert!(router().current_route(),Route::Home);
/// ```
///
///
#[proc_macro]
pub fn add_router(_item: TokenStream) -> TokenStream {
    "thread_local! {
    pub(crate) static ROUTER: Router<Route> = Router::new();
}

/// Access the router from global state
fn router() -> Router<Route,> {
    ROUTER.with(Clone::clone,)
}"
    .parse()
    .unwrap()
}
/// Derive an enum as Routing for navigation
/// You can change the value of a path for a given route this way
///
///
///
/// ```rust
/// #[derive(Debug, PartialEq, Copy, Clone, ParseUrl)]
/// pub enum DashboardAdminRoutes {
///     #[as_path = "my_stuff"] // "/my_stuff"
///     Other,
///     #[as_path = ""]
///     Root, // "/"
/// }
///
/// fn test_url() {
///     let mut query_search: IndexMap<String, String> = IndexMap::new();
///
///     query_search.insert("user".to_string(), "arn".to_string());
///     query_search.insert("role".to_string(), "baby_programmer".to_string());
///     query_search.insert("location".to_string(), "norway".to_string());
///     let url = ExampleRoutes::Admin {
///         query: query_search.clone(),
///     }
///     .to_url();
///     let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
///         .parse()
///         .unwrap();
///     assert_eq!(url, url_to_compare);
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(ParseUrl, attributes(as_path))]
pub fn derive_as_url(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };
    let variants = variants.iter();
    let (as_snippets, parse_snippets) = routing_variant_snippets(variants.clone());

    let name = ident.to_string();
    TokenStream::from(quote! {
     impl router::Navigation for #ident {
        fn to_url(&self) -> Url {
         let url : Url =    match self {
                    #(#as_snippets),*
                    }.parse().unwrap();
                    url
        }

        fn from_url(url: Url) -> std::result::Result<Self, ParseError>
         where
        Self: Sized + ParsePath {
        let string_url = url.to_string();
          Self::parse_path(&string_url)
        }
    }
            impl AsPath for #ident {
            fn as_path(self) -> String {
                match self {
                    #(#as_snippets),*
                }
            }
        }
        impl router::ParsePath for #ident {
            fn parse_path(path: &str) -> std::result::Result<Self, ParseError> {
                let next = path.trim_start_matches("/");
                Err(ParseError::NoMatch)
                    #(.or_else(|err|
                        #parse_snippets
                        )
                    )*
                    .map_err(|err| ParseError::By(#name.to_string(), Box::new(err)))
            }
        }
    })
}

/// Define a routing config as root for your navigation.
/// It will contain the default route used by the router when it cannot find the
/// right url
///
/// ```rust
/// #[derive(Debug, PartialEq, Copy, Clone, Root)]
/// pub enum DashboardAdminRoutes {
///     #[default_route]
///     NotFound, // -> /blablablalbla -> /not_found
///     Root,
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(Root, attributes(default_route))]
pub fn define_as_root(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };
    let variants = variants.iter();
    let default_route = get_default_route(variants.clone());

    if default_route.is_err() {
        abort!(Diagnostic::new(
            Level::Error,
            "Could not find default_route".into()
        ))
    }

    let default_variant = default_route.unwrap();

    match default_variant.fields {
        Fields::Named(_) => abort!(Diagnostic::new(
            Level::Error,
            "Default route need to be simple".into()
        )),
        Fields::Unnamed(_) => abort!(Diagnostic::new(
            Level::Error,
            "Default route need to be simple".into()
        )),
        Fields::Unit => {}
    }

    let default_variant_ident = default_variant.ident;

    TokenStream::from(quote! {

      impl Default for #ident {
            fn default() -> #ident {
                #ident::#default_variant_ident
            }
        }

      impl DefaultRoute<#ident> for #ident {
            fn is_default(&self) -> bool {
                self == &#ident::#default_variant_ident
            }
      }
    })
}

/// The RoutingModule makes the enum variants representing modules loaded by the
/// routes.
/// By default, an enum variant snake case is equal to its module name
///
///  You can rename the path
///  You can specify routes that does not load module ( no init, no specific
/// Model & Msg and no view )
///
/// The derive macro will call the init function , Model, Msg, Routes, Update,
/// and View
///
///
///
/// ```rust
/// fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
///     orders
///         .subscribe(Msg::UrlChanged)
///         .subscribe(Msg::UrlRequested)
///         .subscribe(Msg::UserLogged);
///
///     let mut router: Router<Routes> = Router::new();
///     router.init_url_and_navigation(url);
///
///     Model {
///         theme: Theme::default(),
///         register: Default::default(),
///         login: Default::default(),
///         dashboard: Default::default(),
///         admin: Default::default(),
///         router,
///         logged_user: None,
///     }
/// }
///
/// struct Model {
///     pub register: pages::register::Model,
///     pub login: pages::login::Model,
///     pub dashboard: pages::dashboard::Model,
///     pub admin: pages::admin::Model,
///     router: Router<Routes>,
///     logged_user: Option<LoggedUser>,
///     theme: Theme,
/// }
///
///
/// #[derive(Debug, PartialEq, Clone, RoutingModules)]
/// #[modules_path = "pages"]
/// pub enum Routes {
///     Login {
///         query: IndexMap<String, String,>, /// -> http:///localhost:8000/login?name=JohnDoe
///     },
///     #[guard = " => guard => forbidden"]
///     Dashboard(pages::dashboard::Routes,), /// -> http:///localhost:8000/dashboard/*
///     #[guard = " => admin_guard => forbidden_user"]
///     Admin {
///         /// -> /admin/:id/*
///         id: String,
///         children: pages::admin::Routes,
///     },
///     #[default_route]
///     #[view = " => not_found"] /// -> http:///localhost:8000/not_found*
///     NotFound,
///     #[view = " => forbidden"] /// -> http:///localhost:8000/forbidden*
///     Forbidden,
///     #[as_path = ""]
///     #[view = "theme => home"] /// -> http:///localhost:8000/
///     Home,
/// }
///
/// fn guard(model: &Model,) -> Option<bool,> {
///     /// could check local storage, cookie or what ever you want
///     if model.logged_user.is_some() {
///         Some(true,)
///     } else {
///         None
///     }
/// }
///
/// fn admin_guard(model: &Model,) -> Option<bool,> {
///     /// could check local storage, cookie or what ever you want
///     if let Some(user,) = &model.logged_user {
///         match user.role {
///             Role::StandardUser => Some(false,),
///             Role::Admin => Some(true,),
///         }
///     } else {
///         None
///     }
/// }
///
/// fn not_found(_: &Model,) -> Node<Msg,> {
///     div!["404 page not found"]
/// }
///
/// fn forbidden(_: &Model,) -> Node<Msg,> {
///     div!["401 access denied"]
/// }
///
/// fn forbidden_user(model: &Model,) -> Node<Msg,> {
///     if let Some(user,) = &model.logged_user {
///         p![format!(
///             "Sorry {} {} , but you are missing the Admin Role. Ask your administrator for more \
///              information. ",
///             user.first_name, user.last_name
///         )]
///     } else {
///         div!["401"]
///     }
/// }
///
/// fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
///     match msg {
///         Msg::UrlChanged(subs::UrlChanged(url)) => {
///             model.router.confirm_navigation(url);
///             if let Some(current_route) = model.router.current_route.clone() {
///                 current_route.init(model, orders);
///             }
///         } // ...remaining arms
///     }
/// }
///
/// fn view(model: &Model) -> impl IntoNodes<Msg> {
///     vec![
///         header(&model),
///         if let Some(route) = &model.router.current_route {
///             route.view(model)
///         } else {
///             home(&model.theme)
///         },
///     ]
/// }
/// ```
///
///
/// --------------------
/// Todo :
/// - Could add as_module to rename a module
/// - Could generate the code for fn update as well ?
/// --------------------
#[proc_macro_error]
#[proc_macro_derive(
    RoutingModules,
    attributes(as_path, view, guard, default_route, modules_path)
)]
pub fn derive_add_module_load(item: TokenStream) -> TokenStream {
    let add_url = derive_as_url(item.clone());
    let root = define_as_root(item.clone());
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(
            Level::Error,
            "Can only derive AsPath for enums.".into()
        )),
    };

    let url_impl = TokenStream2::from(add_url);
    let default_route_impl = TokenStream2::from(root);
    let variants = variants.iter();

    let modules_path = modules::path(ident.clone(), attrs.iter());

    let modules_snippets = modules_view_snippets(variants.clone(), modules_path.clone());

    let init_snippets = module_init_snippets(variants.clone(), modules_path);
    TokenStream::from(quote! {
    #url_impl

    #default_route_impl

    impl router::View<#ident, Model, Msg> for  #ident {
        fn view(&self, scoped_state: &Model) -> Node<Msg> {
            match self {
                 #(#modules_snippets),*
            }
        }
    }

         impl router::Init<#ident, Model, Msg> for #ident {
        fn init(
            &self,
            previous_state: &mut Model,
            orders: &mut impl Orders<Msg>,
        ) {
            match self {
                #(#init_snippets),*
            }
        }
    }


    })
}
