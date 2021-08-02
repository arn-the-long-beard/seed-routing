use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Error, Field, Lit, LitStr, Meta, MetaNameValue, Result};

/// Rebuild the content of a variant depending of the fields present in the
/// original enum
pub fn build_variant_arguments(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> TokenStream {
    // Todo this code is ugly, need to find something better
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            quote! { id,query,children}
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            quote! { id, query}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            quote! { query , children}
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            quote! { id, children }
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { id }
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            quote! { query}
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            quote! { children }
        }

        (id, query, children) if query.is_none() && id.is_none() & children.is_none() => {
            quote! {}
        }
        (_, _, _) => {
            quote! {}
        }
    }
}

/// Assign only the payload defined by the field in the enu,
pub fn unwrap_url_payload_matching_field(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> TokenStream {
    // Todo this code is ugly, need to find something better
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let sub_enum = &children.clone().expect("Should have extract url item").ty;
            quote! { id : id.expect("Should have extract url item"),query : query.expect("Should have extract url item"),children :  #sub_enum::parse_path(&children.expect("Should have unwrap children route")).expect("Should have parse children Route but failed")}
        }

        (id, query, _) if id.is_some() && query.is_some() => {
            quote! { id : id.expect("Should have extract url item"),query : query.expect("Should have extract url item")}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let sub_enum = &children.clone().expect("Should have extract url item").ty;
            quote! { query : query.expect("Should have extract url item"),children :  #sub_enum::parse_path(&children.expect("Should unwrap children route")).expect("Should have parse children Route")}
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            let sub_enum = &children.clone().expect("Should have extract url item").ty;
            quote! { id : id.expect("Should have extract url item"),children : #sub_enum::parse_path(&children.expect("Should unwrap children route")).expect("Should have parse children Route")}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { id : id.expect("Should have extract url item")}
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            quote! { query : query.expect("Should have extract url item")}
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            let sub_enum = &children.clone().expect("Should have extract url item").ty;
            quote! { children :#sub_enum::parse_path(&children.expect("Should have extract url item").clone()).expect("Should have parse children Route")}
        }

        (_, _, _) => {
            quote! {}
        }
    }
}
pub fn inject_variant_payload_in_function_call(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> String {
    // Todo this code is ugly, need to find something better
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            "id,query,children".to_string()
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            "id,query".to_string()
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            "query,children".to_string()
        }
        (id, query, children) if id.is_some() && children.is_some() && query.is_none() => {
            "id,children".to_string()
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            "id".to_string()
        }
        (id, query, children) if query.is_some() && id.is_none() && children.is_none() => {
            "query".to_string()
        }
        (id, query, children) if query.is_none() && id.is_none() & children.is_some() => {
            "children".to_string()
        }

        (id, query, children) if query.is_none() && id.is_none() & children.is_none() => {
            "".to_string()
        }
        (_, _, _) => "".to_string(),
    }
}

pub fn extract_query_field_to_string() -> TokenStream {
    quote! { convert_to_string(&query)}
}
pub fn build_string_without_path_name(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
) -> TokenStream {
    // Todo this code is ugly, need to find something better
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let query_string = extract_query_field_to_string();
            quote! { format!("{}?{}",  id, children.clone().as_path() , #query_string)}
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            let query_string = extract_query_field_to_string();

            quote! { format!("/{}?{}",  id, #query_string)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let query_string = extract_query_field_to_string();

            quote! { format!("/{}?{}",   children.clone().as_path(),#query_string)}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_some() => {
            quote! { format!("/{}{}", id,  children.clone().as_path())}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { format!("/{}", id)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_none() => {
            let query_string = extract_query_field_to_string();
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

pub fn build_string_with_path_name(
    structs_tuple: (Option<&Field>, Option<&Field>, Option<&Field>),
    name: Option<String>,
) -> TokenStream {
    // Todo this code is ugly, need to find something better
    match structs_tuple {
        (id, query, children) if id.is_some() && query.is_some() && children.is_some() => {
            let query_string = extract_query_field_to_string();
            quote! { format!("/{}/{}{}?{}", #name, id, children.clone().as_path() , #query_string)}
        }

        (id, query, children) if id.is_some() && query.is_some() && children.is_none() => {
            let query_string = extract_query_field_to_string();

            quote! { format!("/{}/{}?{}", #name, id, #query_string)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_some() => {
            let query_string = extract_query_field_to_string();

            quote! { format!("/{}/{}?{}", #name,  children.clone().as_path(),#query_string)}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_some() => {
            quote! { format!("/{}/{}{}", #name, id,  children.clone().as_path())}
        }
        (id, query, children) if id.is_some() && query.is_none() && children.is_none() => {
            quote! { format!("/{}/{}", #name, id)}
        }
        (id, query, children) if id.is_none() && query.is_some() && children.is_none() => {
            let query_string = extract_query_field_to_string();
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

/// Get the content of an attribute
/// For example it will get the string `home` from `#[as_path="home"]` and the
/// content as a list from `#[view="=>my_view"]` and `#[guard="user =>
/// admin_guard => forbidden_view"]`
pub fn get_string_from_attribute(attribute_name: &str, attr: &Attribute) -> Result<Option<LitStr>> {
    if !attr.path.is_ident(attribute_name) {
        return Ok(None); // not our attribute
    }
    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(name),
            ..
        }) => Some(Some(name)),
        _ => None,
    }
    .ok_or_else(|| Error::new_spanned(attr, &format!("expected #[{} = \"...\"]", attribute_name)))
}
