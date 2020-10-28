use crate::{build_advanced, build_structs, get_string_from_attribute};
use convert_case::{Case, Casing};
use proc_macro_error::{abort, Diagnostic, Level};

use quote::quote;
use syn::{export::TokenStream2, punctuated::Iter, Attribute, Field, Fields, Ident, Variant};

pub fn routing_variant_snippets(
    variants: Iter<'_, Variant>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let name = variant_path_segment(ident.clone(), attrs.iter());
        match fields {
            Fields::Unit => {
                if name.is_none() && (i + 1) != len {
                    abort!(Diagnostic::new(
                        Level::Error,
                        "Unit variant without a name must be declared last.".into()
                    ))
                }
                unit_variant_snippets(ident.clone(), name)
            }
            Fields::Unnamed(fields) => {
                tuple_variant_snippets(ident.clone(), name, fields.unnamed.iter())
            }
            Fields::Named(fields) => {
                struct_variant_snippets(ident.clone(), name, fields.named.iter())
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
fn unit_variant_snippets(ident: Ident, name: Option<String>) -> (TokenStream2, TokenStream2) {
    (
        as_unit_variant(ident.clone(), name.clone()),
        parse_unit_variant(ident, name),
    )
}
fn as_unit_variant(ident: Ident, name: Option<String>) -> TokenStream2 {
    let format = match name {
        Some(name) => quote! { format!("/{}", #name) },
        None => quote! { String::new() },
    };
    quote! {
        Self::#ident => #format
    }
}

fn parse_unit_variant(ident: Ident, name: Option<String>) -> TokenStream2 {
    let parser = match name {
        Some(name) => quote! {
            next.strip_prefix(#name).ok_or(err)
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
    name: Option<String>,
    fields: Iter<'_, Field>,
) -> (TokenStream2, TokenStream2) {
    (
        as_struct_variant(ident.clone(), name.clone(), fields.clone()),
        parse_struct_variant(ident, name, fields),
    )
}
fn as_tuple_variant(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }
    let format = match name {
        Some(name) => quote! { format!("/{}{}", #name, nested.clone().as_path()) },
        None => quote! { nested.as_path() },
    };
    quote! {
        Self::#ident(nested) => #format
    }
}

fn as_struct_variant(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> TokenStream2 {
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

    let structs = build_structs(structs_tuple);

    // let string_enum_with_no_name = build_string(structs_tuple );
    let format = match &name {
        Some(_) => build_string_with_path_name(structs_tuple, name.clone()),
        None => build_string_without_path_name(structs_tuple),
    };
    quote! {
        Self::#ident{#structs} => #format
    }
}

fn build_query() -> TokenStream2 {
    quote! { convert_to_string(query.clone())}
}
fn build_string_without_path_name(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let query_string = build_query();
            quote! { format!("{}?{}",  id, children.clone().as_path() , #query_string)}
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            let query_string = build_query();

            quote! { format!("/{}?{}",  id, #query_string)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let query_string = build_query();

            quote! { format!("/{}?{}",   children.clone().as_path(),#query_string)}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_some() => {
            quote! { format!("/{}{}", id,  children.clone().as_path())}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { format!("/{}", id)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_none() => {
            let query_string = build_query();
            quote! { format!("/?{}",#query_string)}
        }
        (id, query, children) if id.is_none() && query.is_none() && children.is_some() => {
            quote! { format!("/{}",    children.clone().as_path())}
        }

        (_, _, _) => {
            quote! { format!("/")}
        }
    }
}

fn build_string_with_path_name(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
    name: Option<String>,
) -> TokenStream2 {
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let query_string = build_query();
            quote! { format!("/{}/{}{}?{}", #name, id, children.clone().as_path() , #query_string)}
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            let query_string = build_query();

            quote! { format!("/{}/{}?{}", #name, id, #query_string)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let query_string = build_query();

            quote! { format!("/{}/{}?{}", #name,  children.clone().as_path(),#query_string)}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_some() => {
            quote! { format!("/{}/{}{}", #name, id,  children.clone().as_path())}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { format!("/{}/{}", #name, id)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_none() => {
            let query_string = build_query();
            quote! { format!("/{}?{}", #name,#query_string)}
        }
        (id, query, children) if id.is_none() && query.is_none() && children.is_some() => {
            quote! { format!("/{}/{}", #name,   children.clone().as_path())}
        }

        (_, _, _) => {
            quote! { format!("/{}", #name)}
        }
    }
}

fn parse_tuple_variant(
    ident: Ident,
    name: Option<String>,
    fields: Iter<'_, Field>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }

    let parser = match name {
        Some(name) => quote! {
            next.strip_prefix(#name).ok_or(err)
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
    name: Option<String>,
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
    let structs = build_advanced(structs_tuple);
    let parser = match name {
        Some(name) => {
            quote! {      next.strip_prefix(#name).ok_or(err)
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
    let mut attrs = attrs.filter_map(|attr| match get_string_from_attribute("as_path", attr) {
        Ok(op) => op,
        Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
    });
    let name = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Multiple path names defined.".into()
        ))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        ident.to_string().to_case(Case::Snake)
    };
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
