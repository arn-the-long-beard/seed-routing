/// Return true if the Route is default and used when no path is matching the enumeration.
pub trait DefaultRoute<Route> {
    #[must_use]
    fn is_default(&self) -> bool;
}
