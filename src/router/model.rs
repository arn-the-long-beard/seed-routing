use seed::prelude::Orders;

/// The init on the route will call the init of their modules if they are not
/// local view
pub trait Init<Routes, State, Msg: 'static> {
    fn init(&self, previous_state: &mut State, orders: &mut impl Orders<Msg>);
}
