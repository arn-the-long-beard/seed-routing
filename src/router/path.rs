use std::str::FromStr;

#[allow(clippy::module_name_repetitions)]
pub trait AsPath {
    fn as_path(self) -> String;
}
impl<T: ToString> AsPath for T {
    fn as_path(self) -> String {
        format!("/{}", self.to_string())
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait ParsePath: AsPath + Sized {
    /// Implementation is provided for all types implementing `FromStr`, `ToString` and `AsPath`
    ///
    /// The provided implementation trims all leading `'/'` characters, before running the std parse
    ///
    /// # Errors
    ///
    /// Will return Err if it's not possible to parse this string slice into the desired type.
    fn parse_path(route: &str) -> Result<Self, ParseError>;
}
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
