use seed::prelude::Node;

/// This trait allows you to display specific nodes in function of the current
/// route and guards if needed.
/// View is not recursive. It does load the `local_view` matching `current_route()` or the `fn view` of the submodule.
/// If the sub module contains nested or children routes, you need to call yourself their implementation of view to get the right view with `dashboard_routes.view(model)` as in the [example](https://github.com/arn-the-long-beard/seed-routing/blob/main/examples/backbone_app/src/pages/dashboard/mod.rs)
/// This rule is made to give maximum focus & control on the current module you are working with.
/// # Routes
/// The routes enum that when matching will give a specific view.
/// Nested Routes and children Routes are passed to the view to call view() on
/// them as well.
/// # Model
/// The model will be used by the view to display data in Html.
/// # Msg
/// The standard Msg Enum that we can use to triggers events or actions.
pub trait View<Routes, Model, Msg> {
    #[must_use]
    fn view(&self, scoped_state: &Model) -> Node<Msg>;
}
