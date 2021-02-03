use convert_case::{Case, Casing};
use proc_macro_error::{abort, Diagnostic, Level};

use crate::builder::{
    build_string_with_path_name, build_string_without_path_name, build_variant_arguments,
    get_string_from_attribute, unwrap_url_payload_matching_field,
};
use quote::quote;

use std::collections::HashSet;
use syn::{export::TokenStream2, punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

/// Build the matching arms for the enum match for ParseUrl trait implementation
pub fn routing_variant_snippets(
    variants: Iter<'_, Variant>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let len = variants.len();

    let mut check_hash = HashSet::new();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let path_name = variant_path_segment(ident.clone(), attrs.iter());
        let path_name_as_string = path_name.clone().unwrap_or_else(|| "".to_string());
        if !check_hash.insert(path_name_as_string.clone()) {
            abort!(Diagnostic::new(
                Level::Error,
                format!(
                    "A Path must be unique and '{}' is found multiple times",
                    path_name_as_string
                )
                .into()
            ))
        }

        match fields {
            Fields::Unit => {
                // enforces variant with empty-string path is last variant
                if path_name.is_none() && (i + 1) != len {
                    abort!(Diagnostic::new(
                        Level::Error,
                        "Unit variant without a name must be declared last.".into()
                    ))
                }

                unit_variant_snippets(ident.clone(), path_name)
            }
            Fields::Unnamed(fields) => {
                if path_name.is_none() {
                    abort!(Diagnostic::new(
                        Level::Error,
                        "Empty path are not allowed in Routes with argument.".into()
                    ))
                }
                tuple_variant_snippets(ident.clone(), path_name, fields.unnamed.iter())
            }
            Fields::Named(fields) => {
                if path_name.is_none() {
                    abort!(Diagnostic::new(
                        Level::Error,
                        "Empty path are not allowed in Routes with arguments.".into()
                    ))
                }
                struct_variant_snippets(ident.clone(), path_name, fields.named.iter())
            }
        }
    });
    snippets.fold(
        (Vec::with_capacity(len), Vec::with_capacity(len)),
        |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        },
    )
}
fn unit_variant_snippets(ident: Ident, path_name: Option<String>) -> (TokenStream2, TokenStream2) {
    (
        as_unit_variant(ident.clone(), path_name.clone()),
        parse_unit_variant(ident, path_name),
    )
}

// if with `as_path = "foo"` set's up to get "/foo"
// if no `as_path` gives togens to
fn as_unit_variant(ident: Ident, path_name: Option<String>) -> TokenStream2 {
    let format = match path_name {
        Some(path_name) => quote! { format!("/{}", #path_name) },
        None => quote! { String::new() },
    };
    quote! {
        Self::#ident => #format
    }
}

fn parse_unit_variant(ident: Ident, path_name: Option<String>) -> TokenStream2 {
    let parser = match path_name {
        Some(path_name) => quote! {
            next.strip_prefix(#path_name).ok_or(err)
        },
        None => quote! {
            if next.is_empty() {
                Some(())
            } else {
                None
            }
            .ok_or(ParseError::RemainingSegments)
        },
    };
    quote! {
        #parser.map(|_| Self::#ident)
    }
}
fn tuple_variant_snippets(
    ident: Ident,
    name: Option<String>,
    fields: Iter<'_, Field>,
) -> (TokenStream2, TokenStream2) {
    (
        as_tuple_variant(ident.clone(), name.clone(), fields.clone()),
        parse_tuple_variant(ident, name, fields),
    )
}

fn struct_variant_snippets(
    ident: Ident,
    path_name: Option<String>,
    fields: Iter<'_, Field>,
) -> (TokenStream2, TokenStream2) {
    (
        as_struct_variant(ident.clone(), path_name.clone(), fields.clone()),
        parse_struct_variant(ident, path_name, fields),
    )
}
fn as_tuple_variant(
    ident: Ident,
    path_name: Option<String>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }
    let format = match path_name {
        Some(path_name) => quote! { format!("/{}{}", #path_name, nested.clone().as_path()) },
        None => quote! { nested.as_path() },
    };
    quote! {
        Self::#ident(nested) => #format
    }
}

fn as_struct_variant(
    ident: Ident,
    path_name: Option<String>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
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

    // let string_enum_with_no_name = build_string(structs_tuple );
    let format = match &path_name {
        Some(_) => build_string_with_path_name(structs_tuple, path_name.clone()),
        None => build_string_without_path_name(structs_tuple),
    };
    quote! {
        Self::#ident{#structs} => #format
    }
}

fn parse_tuple_variant(
    ident: Ident,
    path_name: Option<String>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }

    let parser = match path_name {
        Some(path_name) => quote! {
            next.strip_prefix(#path_name).ok_or(err)
                .and_then(|rest|
                    ParsePath::parse_path(rest)
                )
        },
        None => quote! {
            ParsePath::parse_path(next)
        },
    };
    quote! {
        #parser.map( Self::#ident )
    }
}
fn parse_struct_variant(
    ident: Ident,
    path_name: Option<String>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    // let children = fields.find(|f| f.ident.as_ref().unwrap() == "children");
    let id_param = fields.clone().find(|f| f.ident.as_ref().unwrap() == "id");
    let query_parameters = fields
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "query");

    // update when having children available.
    let children = fields
        .clone()
        .find(|f| f.ident.as_ref().unwrap() == "children");

    let structs_tuple = (id_param, query_parameters, children);

    let with_id_param = structs_tuple.0.is_some();
    let with_query_params = structs_tuple.1.is_some();
    let with_children = structs_tuple.2.is_some();
    let structs = unwrap_url_payload_matching_field(structs_tuple);
    let parser = match path_name {
        Some(path_name) => {
            quote! {      next.strip_prefix(#path_name).ok_or(err)
                     .map(|rest| extract_url_payload(rest.to_string(),#with_id_param,#with_query_params,#with_children ))
            }
        }
        None => quote! {
             next.strip_prefix("").ok_or(err)
            .map(|rest| extract_url_payload(rest.to_string(),#with_id_param,#with_query_params,#with_children ))
        },
    };

    quote! {
        #parser.map(|(id, query, children)| Self::#ident{#structs})
    }
}
fn variant_path_segment(ident: Ident, attrs: std::slice::Iter<'_, Attribute>) -> Option<String> {
    // get just "as_path" attributes of an identity
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("as_path", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });

    // make sure only one path specified for each variant
    let path_name = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple path names defined.".into()
        ))
    // we have an as_path = "...", get the "..."
    } else if let Some(path_name) = attrs.next() {
        path_name.value()
    } else {
        // no path specified, so use default (snake case version of ident)
        ident.to_string().to_case(Case::Snake)
    };

    if path_name.is_empty() {
        None
    } else if path_name.contains('/') {
        abort!(Diagnostic::new(
            Level::Error,
            "A renamed path cannot contain the character '/' ".into()
        ))
    } else {
        Some(path_name)
    }
}
