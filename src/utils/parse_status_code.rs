use syn::{Attribute, LitInt, Token};

pub fn parse_status_code(attrs: &[Attribute]) -> syn::Result<Option<u16>> {
    let mut status_code = None;

    for attr in attrs {
        if attr.path().is_ident("into_response") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("status") {
                    meta.input.parse::<Token![=]>()?;
                    let lit = meta.input.parse::<LitInt>()?;
                    let status = lit.base10_parse::<u16>()?;
                    status_code = Some(status);
                    Ok(())
                } else {
                    Err(meta.error("unrecognized into_response attribute"))
                }
            })?;
        }
    }
    Ok(status_code)
}
