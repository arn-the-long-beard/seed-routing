use seed::prelude::Node;

/// This trait allow you to display specific nodes in function of the current
/// route and guards if needed
///
/// # Routes
/// The routes enum that when matching will give view a specific view
/// Nested Routes and children Routes are passed to the view to call view() on
/// them as well
/// # Model
/// The model will be used by the view to display data in Html
/// # Msg
/// The standard Msg Enum that we can use to triggers events or actions
///
/// TODO :
/// _______________________________________________________________________________
/// Should we pass query and id parameter to view as well or having it in
/// init is enough ?
/// _______________________________________________________________________________
pub trait View<Routes, Model, Msg> {
    fn view(&self, scoped_state: &Model) -> Node<Msg>;
}

// pub trait Guarded<Routes, State, Msg> {
//     fn check_before_load(&self, scoped_state: &State) -> Option<bool>;
// }
