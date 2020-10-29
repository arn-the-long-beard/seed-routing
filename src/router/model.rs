use seed::prelude::Orders;

pub trait Init<Routes, State, Msg: 'static> {
    fn init(&self, previous_state: &mut State, orders: &mut impl Orders<Msg>);
}
