use proc_macro_error::{abort, Diagnostic, Level};

use syn::{parse::Result, punctuated::Iter, Attribute, Ident, Variant};

/// Identify the default route and catch error if none or too many
pub fn get_default_route(variants: Iter<'_, Variant>) -> Result<Variant> {
    let mut i = 0;
    let mut default_variant: Option<Variant> = None;
    for v in variants {
        let default = variant_default_route(v.ident.clone(), &v.attrs);
        if default {
            i += 1;
            default_variant = Some(v.clone());
        }
    }
    if i == 0 {
        abort!(Diagnostic::new(
            Level::Error,
            "You need at least one default route with the attribute #[default_route].".into()
        ));
    } else if i > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "You cannot have multiple default routes.".into()
        ));
    } else {
        Ok(default_variant.unwrap())
    }
}

/// Check if default_route exist
fn variant_default_route(_: Ident, attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.is_ident("default_route"))

}
