use std::str::FromStr;

pub trait AsPath {
    fn as_path(self) -> String;
}
impl<T: ToString> AsPath for T {
    fn as_path(self) -> String {
        format!("/{}", self.to_string())
    }
}

pub trait ParsePath: AsPath + Sized {
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
        path.trim_start_matches("/")
            .parse::<T>()
            .map_err(|_| ParseError::FromStr)
    }
}
