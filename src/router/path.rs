use std::str::FromStr;

/// Convert an enumeration variant to a string. Used as base for a route to be
/// converted to `Url`.
#[allow(clippy::module_name_repetitions)]
pub trait AsPath {
    #[must_use]
    fn as_path(self) -> String;
}
impl<T: ToString> AsPath for T {
    #[must_use]
    fn as_path(self) -> String {
        format!("/{}", self.to_string())
    }
}

/// Parse a url string into a route enum `Variant`.
/// ```rust
/// extern crate router_derive;
/// extern crate seed_routing;
/// use router_derive::*;
/// use seed::prelude::{IndexMap, *};
/// use seed_routing::*;
///
/// #[derive(Debug, PartialEq, Clone, ParseUrl, WithDefaultRoute)]
/// pub enum ExampleRoutes {
///     Other {
///         id: String,
///         children: Settings,
///     },
///     #[default_route]
///     NotFound,
/// }
/// #[derive(Debug, PartialEq, Clone, ParseUrl)]
/// pub enum Settings {
///     Api(Apis),
///     Projects {
///         id: String,
///         query: IndexMap<String, String>,
///         children: Apis,
///     },
/// }
///
/// #[derive(Debug, PartialEq, Clone, ParseUrl)]
/// pub enum Apis {
///     Facebook,
///     Google,
///     Microsoft,
/// }
///
/// let mut query: IndexMap<String, String> = IndexMap::new();
///
/// query.insert("user".to_string(), "arn".to_string());
/// query.insert("role".to_string(), "baby_programmer".to_string());
/// query.insert("location".to_string(), "norway".to_string());
///
/// let string_to_compare =
///     "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway";
/// assert_eq!(
///     ExampleRoutes::parse_path(string_to_compare).unwrap(),
///     ExampleRoutes::Other {
///         id: "2".to_string(),
///         children: Settings::Projects {
///             id: "14".to_string(),
///             query: query.clone(),
///             children: Apis::Facebook
///         },
///     }
/// );
/// ```
///
#[allow(clippy::module_name_repetitions)]
pub trait ParsePath: AsPath + Sized {
    /// Implementation is provided for all types implementing `FromStr`,
    /// `ToString` and `AsPath`
    ///
    /// The provided implementation trims all leading `'/'` characters, before
    /// running the std parse
    ///
    /// # Errors
    ///
    /// Will return Err if it's not possible to parse this string slice into the
    /// desired type.
    fn parse_path(route: &str) -> Result<Self, ParseError>;
}

/// Error enum for parsing from string to `Route` with extra information.
#[derive(Debug)]
pub enum ParseError {
    FromStr,
    NoMatch,
    By(String, Box<ParseError>),
    RemainingSegments,
}
impl<T: FromStr + ToString + AsPath> ParsePath for T {
    fn parse_path(path: &str) -> Result<Self, ParseError> {
        path.trim_start_matches('/')
            .parse::<T>()
            .map_err(|_| ParseError::FromStr)
    }
}
