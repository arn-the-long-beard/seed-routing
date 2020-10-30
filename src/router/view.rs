use seed::prelude::Node;

/// This trait allow you to display specific nodes in function of the current
/// route and guards if needed
pub trait View<Routes, State, Msg> {
    fn view(&self, scoped_state: &State) -> Node<Msg>;
}

// pub trait Guarded<Routes, State, Msg> {
//     fn check_before_load(&self, scoped_state: &State) -> Option<bool>;
// }
