pub mod router;

pub use router::*;
pub use router_macro_derive::{AsUrl, Root, RoutingModules};
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}