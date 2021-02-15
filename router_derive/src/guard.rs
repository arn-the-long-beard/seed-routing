use proc_macro_error::{abort, Diagnostic, Level};

use crate::builder::get_string_from_attribute;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};
/// Extract the value from the [#guard  = "model_prop => guard_function"]
pub fn variant_guard_path_tuple(
    _ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String, String)> {
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("guard", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let guard_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple guard defined are not supported.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if guard_scope.is_empty() {
        None
    } else {
        let string_to_parse = guard_scope;
        let guard_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut guard_scope_string_iter = guard_scope_string.iter();
        let guard_path = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got \
                 this {:?}",
                string_to_parse
            )
        });
        let guard_function = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got \
                 this {:?}",
                string_to_parse
            )
        });
        let guard_redirect = guard_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[guard_path = PATH => GUARD_FUNCTION => REDIRECT_VIEW] but got \
                 this {:?}",
                string_to_parse
            )
        });
        Some((
            guard_path.trim().to_string(),
            guard_function.trim().to_string(),
            guard_redirect.trim().to_string(),
        ))
    }
}

/// Add a guard on the view if guard_scope contains value from [#guard  =
/// "model_prop => guard_function"]
pub fn add_guard_to_view(
    view_to_load: TokenStream,
    guard_scope: (String, String, String),
) -> TokenStream {
    let (model_scope, function_path, redirect) = guard_scope;
    let model_path = if model_scope.is_empty() {
        "scoped_state".to_string()
    } else {
        format!("scoped_state.{}.as_ref()", model_scope)
    };
    let redirect_token: TokenStream = format!(" {}({})", redirect, model_path).parse().unwrap();
    let guard_function_token: TokenStream = format!("{}({})", function_path, model_path)
        .parse()
        .unwrap();
    quote! {
         if let Some(authenticated) = #guard_function_token {
               if authenticated {
                  #view_to_load
                }
                else {
                  #redirect_token
                }
            } else {
               #redirect_token
            }

    }
}
