use proc_macro2::{Span, TokenStream};
use syn::{Error, ItemStruct, Result};

use crate::{impl_variant, VariantOf};

pub fn doit(item_struct: ItemStruct) -> Result<TokenStream> {
    let VariantOf {
        enum_path,
        variant_ident,
    } = item_struct
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("variant_of"))
        .map(|attr| attr.parse_args::<VariantOf>())
        .transpose()?
        .ok_or_else(|| Error::new(Span::call_site(), "Variant require #[variant_of] attribute"))?;

    Ok(impl_variant(
        &item_struct,
        &enum_path,
        variant_ident.as_ref(),
    ))
}
