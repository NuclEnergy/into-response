use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use utils::parse_status_code::parse_status_code;

mod utils;

/// # Examples
///
/// ### Simple usage
/// ```rust
/// use into_response::IntoResponse;
///
/// #[derive(IntoResponse)]
/// struct MyResponse {
///     message: String,
/// }
///
/// let response = MyResponse {
///     message: "Hello, world!".to_string(),
/// };
///
/// let response = response.into_response();
///
/// assert_eq!(response.status(), axum::http::StatusCode::OK);
/// ```
///
/// ---
///
/// ### You can also specify a custom status code using the `into_response` attribute
/// ```rust
/// use into_response::IntoResponse;
///
/// #[derive(IntoResponse)]
/// #[into_response(status = 201)]
/// struct MyResponse2 {
///     message: String,
/// }
///
/// let response = MyResponse2 {
///     message: "Hello, world!".to_string(),
/// };
///
/// let response = response.into_response();
///
/// assert_eq!(response.status(), axum::http::StatusCode::CREATED);
/// ```
#[proc_macro_derive(IntoResponse, attributes(into_response))]
pub fn into_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let attrs = &input.attrs;

    let custom_status_code = match parse_status_code(attrs) {
        Ok(code) => code,
        Err(e) => return e.to_compile_error().into(),
    };

    let status_code_tokens = if let Some(code) = custom_status_code {
        quote! { Some(axum::http::StatusCode::from_u16(#code).unwrap())}
    } else {
        quote! { None }
    };

    let mut generics_with_bounds = generics.clone();
    for type_param in &mut generics_with_bounds.type_params_mut() {
        type_param.bounds.push(syn::parse_quote!(serde::Serialize));
    }

    let (impl_generics, ty_generics, where_clause) = generics_with_bounds.split_for_impl();

    let expanded = quote! {
        impl #impl_generics axum::response::IntoResponse for #name #ty_generics #where_clause {
            fn into_response(self) -> axum::response::Response {
                let json = axum::Json(self);
                match #status_code_tokens {
                    Some(status_code) => {
                        let response: (axum::http::StatusCode, axum::Json<Self>) = (status_code, json);
                        response.into_response()
                    },
                    None => json.into_response(),
                }
            }
        }
    };

    expanded.into()
}
