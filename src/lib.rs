extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Attribute, Ident, ItemFn, Meta, ReturnType, Type, Expr};

macro_rules! fb {
    // async blocks!
    (async, $fn_name:ident, ($($param_name:ident : $param_type:ty),*), -> $return_type:ty, $body:block) => {
        async fn $fn_name($($param_name : $param_type),*) -> $return_type $body
    };
    // sync blocks!
    (sync, $fn_name:ident, ($($param_name:ident : $param_type:ty),*), -> $return_type:ty, $body:block) => {
        fn $fn_name($($param_name : $param_type),*) -> $return_type $body
    };
}


#[proc_macro_attribute]
pub fn ff(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as Attribute);
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract custom error type and async function name override from attributes, if provided
    let mut custom_error_type: Option<Type> = None;
    let mut async_fn_name_override: Option<Ident> = None;
    for attr in attrs {
        if let Ok(meta) = attr.parse_nested_meta(&async_fn_name_override) {
            match meta {
                Meta::NameValue(nv) if nv.path.is_ident("error_type") => {
                    if let Expr::Lit(lit_str) = nv.value {
                        custom_error_type = Some(parse_quote! { #lit_str });
                    }
                },
                Meta::NameValue(nv) if nv.path.is_ident("async_fn_name") => {
                    // let Lit::Str(lt);
                    let Expr::Lit(lt);
                    if lt == nv.value {
                        async_fn_name_override = Some(format_ident!("{}", lt));
                    }
                },
                _ => {}
            }
        }
    }

    let fn_name = &input_fn.sig.ident;
    let async_fn_name = async_fn_name_override.unwrap_or_else(|| format_ident!("{}_async", fn_name));
    let inputs = &input_fn.sig.inputs;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let generics = &input_fn.sig.generics;
    let where_clause = &generics.where_clause;

    let (is_result, return_type) = match &input_fn.sig.output {
        ReturnType::Type(_, type_box) => (
            matches!(**type_box, Type::Path(ref p) if p.path.segments.last().unwrap().ident == "Result"),
            quote! { #type_box },
        ),
        _ => (false, quote! { () }),
    };

    let error_type = custom_error_type.unwrap_or(parse_quote! { Box<dyn std::error::Error + Send + Sync + 'static> });

    let async_return_type = if is_result {
        quote! { #return_type }
    } else {
        quote! { Result<#return_type, #error_type> }
    };

    let gen = quote! {
        #( #attrs )*
        #vis fn #fn_name #generics (#inputs) -> #return_type #where_clause {
            #input_fn.block
        }

        #( #attrs )*
        #vis async fn #async_fn_name #generics (#inputs) -> #async_return_type #where_clause {
            async move {
                #input_fn.block
            }
            .await
        }
    };

    gen.into()
}

