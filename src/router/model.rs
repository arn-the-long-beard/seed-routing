use seed::prelude::Orders;

/// The init on the route will call the init of their modules if they are not
/// local view
///
///
/// # Routes
/// The routes enum that when matching will give a specific view
/// If the route contains payload ( query, id or children ) it will be passed to
/// the init call so you can use it to init children routes or make state
/// management.
/// # Model
/// The standard model in TEA
/// The model will be used by the init called and get updated
/// # Msg
/// The standard Msg Enum that we can use to triggers events or actions
pub trait Init<Routes, Model, Msg: 'static> {
    fn init(&self, previous_state: &mut Model, orders: &mut impl Orders<Msg>);
}
