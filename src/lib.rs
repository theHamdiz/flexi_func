extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input,  ItemFn, ReturnType, AttributeArgs, NestedMeta, Meta, Lit};

#[allow(unused_macros)]
/// The `fb` macro (Flexi Block) *or* (Function Builder) simplifies the generation of conditional synchronous or asynchronous functions within Rust code.
///
/// By specifying a mode, function name, parameters, return type, and body, this macro can dynamically create the desired function type based on the provided mode. This approach is particularly useful in contexts where both synchronous and asynchronous versions of a function might be needed, allowing for cleaner code organization and reuse.
///
/// # Syntax
///
/// ```
/// fb!(mode, function_name, (parameter1: Type1, parameter2: Type2, ...), -> ReturnType, {
///     // Function body
/// });
/// ```
///
/// # Parameters
///
/// - `mode`: A compile-time string literal that determines whether the generated function is synchronous (`"sync"`) or asynchronous (`"async"`).
/// - `function_name`: The identifier for the generated function.
/// - `parameters`: A comma-separated list of function parameters in the form `parameter_name: Type`.
/// - `ReturnType`: The return type of the function.
/// - `body`: The block of code that defines the function body.
///
/// # Usage
///
/// Generating a synchronous function:
///
/// ```
/// fb!("sync", greet, (name: String), -> String, {
///     format!("Hello, {}", name)
/// });
/// ```
///
/// Generating an asynchronous function:
///
/// ```
/// fb!("async", fetch_data, (url: String), -> Result<String, reqwest::Error>, {
///     reqwest::get(&url).await?.text().await
/// });
/// ```
///
/// # Tricks and Advanced Usage
///
/// ## Conditional Compilation
///
/// The `fb` macro can be combined with Rust's conditional compilation features to selectively compile either the synchronous or asynchronous version of a function based on feature flags or target environment.
///
/// Example with feature flags:
///
/// ```
/// #[cfg(feature = "async")]
/// fb!("async", process_data, (data: Vec<u8>), -> Result<(), MyError>, {
///     // Asynchronous processing
/// });
///
/// #[cfg(not(feature = "async"))]
/// fb!("sync", process_data, (data: Vec<u8>), -> Result<(), MyError>, {
///     // Synchronous processing
/// });
/// ```
///
/// ## Leveraging Macros for DRY Principles
///
/// You can define a wrapper macro around `fb` to reduce repetition when declaring similar functions in different modes. This is especially handy when you have a set of functions that need to be available in both synchronous and asynchronous forms.
///
/// Example:
///
/// ```
/// macro_rules! define_greeting_fn {
///     ($mode:tt) => {
///         fb!($mode, greet, (name: String), -> String, {
///             format!("Hello, {}", name)
///         });
///     };
/// }
///
/// // Now, you can easily generate both versions with minimal repetition:
/// define_greeting_fn!("sync");
/// define_greeting_fn!("async");
/// ```
///
/// By leveraging the `fb` macro in your Rust projects, you can maintain cleaner and more maintainable codebases, especially when dealing with the complexities of synchronous and asynchronous programming patterns.

// Improved macro definition for clarity and simplicity
macro_rules! fb {
    // Unified handling for both async and sync blocks
    ($mode:tt, $fn_name:ident, ($($param_name:ident : $param_type:ty),*), -> $return_type:ty, $body:block) => {{
        match stringify!($mode) {
            "async" => quote! { async fn $fn_name($($param_name : $param_type),*) -> $return_type $body },
            _ => quote! { fn $fn_name($($param_name : $param_type),*) -> $return_type $body },
        }
    }};
}



/// The `ff` macro (Flexi Function) simplifies the generation of both synchronous and asynchronous versions of a function in Rust.
/// By transforming a synchronous function into both synchronous and asynchronous versions.
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
/// use flexi_func::ff; // Assume your_crate is the name of the crate where the ff macro is defined.
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
