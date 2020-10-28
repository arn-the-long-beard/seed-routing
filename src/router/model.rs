use seed::prelude::Orders;

pub trait Init<Routes, State, Msg: 'static> {
    fn init<'b, 'c>(
        &self,
        previous_state: &'b mut State,
        orders: &'c mut impl Orders<Msg>,
    ) -> &'b mut State;
}
