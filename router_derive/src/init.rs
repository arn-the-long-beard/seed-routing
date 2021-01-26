use convert_case::{Case, Casing};

use proc_macro_error::{abort, Diagnostic, Level};

use crate::{
    builder::{build_variant_arguments, inject_variant_payload_in_function_call},
    view::variant_local_view_tuple,
};
use quote::quote;
use syn::{export::TokenStream2, punctuated::Iter, Field, Fields, Ident, Variant};

pub fn module_init_snippets(
    variants: Iter<'_, Variant>,
    modules_path: Option<String>,
) -> Vec<TokenStream2> {
    let len = variants.len();
    let snippets = variants.enumerate().map(|(_, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let view_scope = variant_local_view_tuple(ident.clone(), attrs.iter());

        match fields {
            Fields::Unit => init_for_unit_variant(ident.clone(), view_scope, modules_path.clone()),
            Fields::Unnamed(fields) => init_for_tuple_variant(
                ident.clone(),
                view_scope,
                fields.unnamed.iter(),
                modules_path.clone(),
            ),
            Fields::Named(fields) => init_for_init_struct_variant(
                ident.clone(),
                view_scope,
                fields.named.iter(),
                modules_path.clone(),
            ),
        }
    });
    snippets.fold(Vec::with_capacity(len), |mut acc, x| {
        acc.push(x);
        acc
    })
}

fn init_for_unit_variant(
    ident: Ident,
    local_view: Option<(String, String)>,
    modules_path: Option<String>,
) -> TokenStream2 {
    // Do stuff about nested init maybe ?
    let init_to_load = match local_view {
        Some((_, _)) => {
            quote! { {} }
        }
        None => get_init_token(ident.clone(), modules_path),
    };
    quote! {
        Self::#ident => #init_to_load
    }
}

/// Get the init function token if not local view
fn get_init_token(ident: Ident, modules_path: Option<String>) -> TokenStream2 {
    let module_name = ident.to_string().to_case(Case::Snake);

    let full_path = if let Some(modules_path) = modules_path {
        format!("{}::{}", modules_path, module_name)
    } else {
        module_name.clone()
    };
    let token: TokenStream2 = format!(
        " previous_state.{} = {}::init(self.to_url(),
                    &mut previous_state.{},
                        &mut orders.proxy(Msg::{}),)  ",
        module_name,
        full_path,
        module_name,
        ident.to_string()
    )
    .parse()
    .unwrap();
    quote! {
    #token  }
}

fn init_for_tuple_variant(
    ident: Ident,
    local_view: Option<(String, String)>,
    fields: Iter<'_, Field>,
    modules_path: Option<String>,
) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(
            Level::Error,
            "Tuple variants may only have a single field.".into()
        ))
    }

    // Do stuff about nested init maybe ?
    let init_to_load = match local_view {
        Some((_, _)) => {
            quote! { {} }
        }
        None => {
            let module_name = ident.to_string().to_case(Case::Snake);
            let full_path = if let Some(modules_path) = modules_path {
                format!("{}::{}", modules_path, module_name)
            } else {
                module_name.clone()
            };
            let token: TokenStream2 = format!(
                " previous_state.{} = {}::init(self.to_url(),
                    &mut previous_state.{},
                        nested,
                        &mut orders.proxy(Msg::{}),)  ",
                module_name,
                full_path,
                module_name,
                ident.to_string()
            )
            .parse()
            .unwrap();
            quote! {
            #token  }
        }
    };
    quote! {
        Self::#ident(nested) => #init_to_load
    }
}

fn init_for_init_struct_variant(
    ident: Ident,
    local_view: Option<(String, String)>,
    fields: Iter<'_, Field>,
    modules_path: Option<String>,
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
    let module_name = ident.to_string().to_case(Case::Snake);

    // do stuff also for children init maybe
    //  let string_enum = build_string(structs_tuple, name.clone());
    let payload: String = inject_variant_payload_in_function_call(structs_tuple);

    let init_to_load = match local_view {
        Some((_, _)) => {
            quote! {}
        }
        None => {
            let full_path = if let Some(modules_path) = modules_path {
                format!("{}::{}", modules_path, module_name)
            } else {
                module_name.clone()
            };
            let token: TokenStream2 = if payload.is_empty() {
                format!(
                    " previous_state.{} = {}::init(self.to_url(),
                    &mut previous_state.{},
                        &mut orders.proxy(Msg::{}),)  ",
                    module_name,
                    full_path,
                    module_name, // = model name on parent model
                    ident.to_string()
                )
                .parse()
                .unwrap()
            } else {
                format!(
                    " previous_state.{} ={}::init(self.to_url(),
                    &mut previous_state.{},
                    {},
                        &mut orders.proxy(Msg::{}),)  ",
                    module_name,
                    full_path,
                    module_name,
                    payload,
                    ident.to_string()
                )
                .parse()
                .unwrap()
            };
            quote! {
            #token  }
        }
    };

    quote! {
        Self::#ident{#structs} => #init_to_load
    }
}
