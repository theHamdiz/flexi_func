extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input,  ItemFn, ReturnType, AttributeArgs, NestedMeta, Meta, Lit};
use flexi_func_declarative::fb;

#[allow(unused_macros)]
/// The `ff` proc macro (Flexi Function) simplifies the generation of asynchronous versions of a synchronous function in Rust.
/// By transforming a synchronous function into both synchronous and asynchronous versions, where the actual async stuff is included inside the fb! macro.
///
/// This macro takes a synchronous function and generates an asynchronous version of it,
/// alongside the original synchronous function. The asynchronous function is named by appending `_async` to the original function's name, unless an override is specified using the `async_fn_name` attribute. An optional custom error type for the async version can be specified with the `error_type` attribute.
///
/// # Attributes
///
/// - `async_fn_name`: Overrides the default name of the generated asynchronous function.
/// - `error_type`: Specifies a custom error type for the asynchronous function. The type must implement `From` for any error types that the function body can emit.
///
/// # Usage
///
/// ```
/// use flexi_func::ff;
///
/// #[ff]
/// fn example_sync(s: String) -> usize {
///     s.len()
/// }
///
/// // This generates:
/// // fn example_sync(s: String) -> usize { ... }
/// // async fn example_sync_async(s: String) -> Result<usize, Box<dyn std::error::Error + Send + Sync + 'static>> { ... }
/// ```
///
/// ## With Custom Error Type
///
/// ```
/// #[ff(error_type = "MyError")]
/// fn example_with_error(s: String) -> Result<usize, MyError> {
///     Ok(s.len())
/// }
///
/// // This generates:
/// // fn example_with_error(s: String) -> Result<usize, MyError> { ... }
/// // async fn example_with_error_async(s: String) -> Result<usize, MyError> { ... }
/// ```
///
/// ## Specifying Async Function Name
///
/// ```
/// #[ff(async_fn_name = "custom_async")]
/// fn example_custom_name(s: String) -> usize {
///     s.len()
/// }
///
/// // This generates:
/// // fn example_custom_name(s: String) -> usize { ... }
/// // async fn custom_async(s: String) -> Result<usize, Box<dyn std::error::Error + Send + Sync + 'static>> { ... }
/// ```
///
/// # Note
///
/// The macro assumes that the synchronous version of the function does not return a `Result` type. If it does, and no `error_type` attribute is provided, the default error type for the asynchronous version is `Box<dyn std::error::Error + Send + Sync + 'static>`.
///

#[proc_macro_attribute]
pub fn ff(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    // Streamlined attribute processing with direct mapping to variables
    let custom_error_type = attrs.iter().find_map(|attr| match attr {
        NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("error_type") => match &nv.lit {
            Lit::Str(lit_str) => Some(lit_str.parse::<syn::Type>().unwrap()),
            _ => None,
        },
        _ => None,
    });

    let async_fn_name_override = attrs.iter().find_map(|attr| match attr {
        NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("async_fn_name") => match &nv.lit {
            Lit::Str(lit_str) => Some(format_ident!("{}", lit_str.value())),
            _ => None,
        },
        _ => None,
    });

    // Extracting essential components from the input function
    let ItemFn { attrs, vis, sig, block } = input_fn;
    let syn::Signature { ident, inputs, output, generics, .. } = sig;

    // Determining the return type and adjusting for async transformation
    let async_fn_name = async_fn_name_override.unwrap_or_else(|| format_ident!("{}_async", ident));
    let return_type = match output {
        ReturnType::Type(_, type_box) => quote! { #type_box },
        _ => quote! { () },
    };

    let error_type = custom_error_type.unwrap_or_else(|| syn::parse2(quote! { Box<dyn std::error::Error + Send + Sync + 'static> }).expect("Failed to parse error type"));

    let async_return_type = quote! { Result<#return_type, #error_type> };

    // Generating both synchronous and asynchronous versions of the function
    let gen = quote! {
        #( #attrs )*
        #vis fn #ident #generics (#inputs) -> #return_type {
            #block
        }

        #( #attrs )*
        #vis async fn #async_fn_name #generics (#inputs) -> #async_return_type {
            async move {
                #block
            }.await.map_err(|e| e.into())
        }
    };

    gen.into()
}
