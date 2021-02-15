use convert_case::{Case, Casing};

use crate::{
    builder::{build_variant_arguments, get_string_from_attribute},
    guard::{add_guard_to_view, variant_guard_path_tuple},
};
use proc_macro2::TokenStream;
use proc_macro_error::{abort, Diagnostic, Level};
use quote::quote;
use syn::{punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

pub fn modules_view_snippets(
    variants: Iter<'_, Variant>,
    modules_path: Option<String>,
) -> Vec<TokenStream> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(_, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let local_view = variant_local_view_tuple(ident.clone(), attrs.iter());
        let guard_scope = variant_guard_path_tuple(ident.clone(), attrs.iter());

        match fields {
            Fields::Unit => {
                view_as_unit_variant(ident.clone(), local_view, guard_scope, modules_path.clone())
            }
            Fields::Unnamed(fields) => {
                if local_view.is_some() {
                    abort!(Diagnostic::new(
                        Level::Warning,
                        format!(
                            "Arguments for {} Route are not implemented on local view for now.",
                            ident.clone()
                        )
                    ))
                }
                view_as_tuple_variant(
                    ident.clone(),
                    local_view,
                    guard_scope,
                    fields.unnamed.iter(),
                    modules_path.clone(),
                )
            }
            Fields::Named(fields) => {
                if local_view.is_some() {
                    abort!(Diagnostic::new(
                        Level::Warning,
                        format!(
                            "Arguments for {} Route are not implemented on local view for now.",
                            ident.clone()
                        )
                    ))
                }
                view_as_struct_variant(
                    ident.clone(),
                    local_view,
                    guard_scope,
                    fields.named.iter(),
                    modules_path.clone(),
                )
            }
        }
    });
    snippets.fold(Vec::with_capacity(len), |mut acc, x| {
        acc.push(x);
        acc
    })
}

/// Get the content of #[view = model_prop => view_function]
/// The extracted view is a local to the module it is declared
pub fn variant_local_view_tuple(
    _ident: Ident,
    attrs: std::slice::Iter<'_, Attribute>,
) -> Option<(String, String)> {
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("view", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let view_scope = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple state path defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        "".to_string()
    };
    if view_scope.is_empty() {
        None
    } else {
        let string_to_parse = view_scope;
        let view_scope_string: Vec<&str> = string_to_parse.split("=>").collect();
        let mut view_scope_string_iter = view_scope_string.iter();
        let view_path = view_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect path for  #[view_path = MODEL_PROP => VIEW] but got this {:?}",
                string_to_parse
            )
        });
        let view_init = view_scope_string_iter.next().unwrap_or_else(|| {
            panic!(
                "expect view for  #[view_path = MODEL_PROP => VIEW] but got this {:?}",
                string_to_parse
            )
        });
        Some((view_path.trim().to_string(), view_init.trim().to_string()))
    }
}

/// Generate token
///
///
/// {}(&scoped_state)
/// or
/// {}(&scoped_state.{}
fn get_view_path_token(path: String, view: String) -> TokenStream {
    let token: TokenStream = if path.is_empty() {
        format!(" {}(&scoped_state)", view).parse().unwrap()
    } else {
        format!(" {}(&scoped_state.{})", view, path,)
            .parse()
            .unwrap()
    };

    quote! {
    #token  }
}

fn view_as_unit_variant(
    ident: Ident,
    view_scope: Option<(String, String)>,
    guard_scope: Option<(String, String, String)>,
    modules_path: Option<String>,
) -> TokenStream {
    let module_name = ident.to_string().to_case(Case::Snake);
    let view_to_load = if let Some((path, view)) = view_scope {
        get_view_path_token(path, view)
    } else {
        let full_path = if let Some(modules_path) = modules_path {
            format!("{}::{}", modules_path, module_name)
        } else {
            module_name.clone()
        };

        let token: TokenStream = format!(
            "{}::view( &scoped_state.{}).map_msg(Msg::{})",
            full_path,
            module_name,
            ident.to_string(),
        )
        .parse()
        .unwrap();
        quote! {
        #token  }
    };

    let view_with_guard_or_not = if let Some(guard) = guard_scope {
        add_guard_to_view(view_to_load, guard)
    } else {
        view_to_load
    };

    quote! {
        Self::#ident => #view_with_guard_or_not
    }
}
fn view_as_tuple_variant(
    ident: Ident,
    view_scope: Option<(String, String)>,
    guard_scope: Option<(String, String, String)>,
    fields: Iter<'_, Field>,
    modules_path: Option<String>,
) -> TokenStream {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }
    let module_name = ident.to_string().to_case(Case::Snake);

    let view_to_load = if let Some((path, view)) = view_scope {
        get_view_path_token(path, view)
    } else {
        let full_path = if let Some(modules_path) = modules_path {
            format!("{}::{}", modules_path, module_name)
        } else {
            module_name.clone()
        };
        let token: TokenStream = format!(
            " {}::view(nested, &scoped_state.{}).map_msg(Msg::{}) ",
            full_path,
            module_name,
            ident.to_string(),
        )
        .parse()
        .unwrap();
        quote! {
        #token  }
    };

    let view_with_guard_or_not = if let Some(guard) = guard_scope {
        add_guard_to_view(view_to_load, guard)
    } else {
        view_to_load
    };
    quote! {
            Self::#ident(nested) => # view_with_guard_or_not
    }
}

fn view_as_struct_variant(
    ident: Ident,
    local_view: Option<(String, String)>,
    guard_scope: Option<(String, String, String)>,
    fields: Iter<'_, Field>,
    modules_path: Option<String>,
) -> TokenStream {
    let fields_to_extract = fields.clone();

    let query_parameters = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query");

    let id_param = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "id");

    let children = fields_to_extract
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "children");

    let structs_tuple = (id_param, query_parameters, children);

    let structs = build_variant_arguments(structs_tuple);

    // do stuff also for children init maybe
    // let string_enum = build_string(structs_tuple, name.clone());
    let module_name = ident.to_string().to_case(Case::Snake);

    let view_to_load = if let Some((path, view)) = local_view {
        get_view_path_token(path, view)
    } else {
        let full_path = if let Some(modules_path) = modules_path {
            format!("{}::{}", modules_path, module_name)
        } else {
            module_name.clone()
        };
        let token: TokenStream = if children.is_some() {
            format!(
                " {}::view(&children,&scoped_state.{}).map_msg(Msg::{})",
                full_path,
                module_name,
                ident.to_string(),
            )
            .parse()
            .unwrap()
        } else {
            format!(
                " {}::view(&scoped_state.{}).map_msg(Msg::{})",
                full_path,
                module_name,
                ident.to_string(),
            )
            .parse()
            .unwrap()
        };

        quote! {
        #token  }
    };

    let view_with_guard_or_not = if let Some(guard) = guard_scope {
        add_guard_to_view(view_to_load, guard)
    } else {
        view_to_load
    };

    quote! {
        Self::#ident{#structs} => #view_with_guard_or_not
    }
}
