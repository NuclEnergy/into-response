use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// # Examples
///
/// ```
/// use into_response::IntoResponse;
///
/// #[derive(IntoResponse)]
/// struct MyResponse {
///     message: String,
/// }
/// ```
#[proc_macro_derive(IntoResponse)]
pub fn into_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                axum::Json(&self).into_response()
            }
        }
    };
    gen.into()
}
