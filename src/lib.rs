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
    let generics = &input.generics;

    let mut generics_with_bounds = generics.clone();
    for type_param in &mut generics_with_bounds.type_params_mut() {
        type_param.bounds.push(syn::parse_quote!(serde::Serialize));
    }

    let (impl_generics, ty_generics, where_clause) = generics_with_bounds.split_for_impl();

    let expanded = quote! {
        impl #impl_generics axum::response::IntoResponse for #name #ty_generics #where_clause {
            fn into_response(self) -> axum::response::Response {
                axum::Json(&self).into_response()
            }
        }
    };

    expanded.into()
}
