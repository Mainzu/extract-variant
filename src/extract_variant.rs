use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Error, ItemEnum, Path, Result, Variant};

use crate::{generate_variant, impl_variant};

type Prefix = (Attribute, Ident);
type Suffix = (Attribute, Ident);
type NoImpl = (Attribute,);

#[derive(Default)]
struct Config {
    prefix: Option<Prefix>,
    suffix: Option<Suffix>,
    no_impl: Option<NoImpl>,
}

pub fn doit(item_enum: ItemEnum) -> Result<TokenStream> {
    // If the ItemEnum has generic parameters, return a compile-time error
    if let Some(lt_token) = item_enum.generics.lt_token {
        return Err(Error::new_spanned(
            lt_token,
            "`extract_variant` does not support generic parameters",
        ));
    }

    let mut config = Config::default();

    for attr in &item_enum.attrs {
        if let Some(ident) = attr.path.get_ident() {
            match ident.to_string().as_str() {
                "prefix" => config.fill_prefix(|| attr.parse_args().map(|p| (attr.clone(), p)))?,
                "suffix" => config.fill_suffix(|| attr.parse_args().map(|p| (attr.clone(), p)))?,
                "no_impl" => config.fill_no_impl(|| Ok((attr.clone(),)))?,
                _ => {}
            }
        }
    }

    let prefix = config
        .prefix
        .map(|(_, id)| id.to_string())
        .unwrap_or_default();
    let suffix = config
        .suffix
        .map(|(_, id)| id.to_string())
        .unwrap_or_default();
    let no_impl = config.no_impl.is_some();

    // Keep track of the path to the enum to generate conversion impls
    // AI: Create a path to the enum using its identifier
    //
    // The reason it's a path and not just an identifier is because the function that generate the
    // implementations is general. This is to support `variant_of`.
    let enum_path = Path::from(item_enum.ident.clone());

    // Create a closure to generate modified variant names if prefix or suffix is non-empty
    let struct_name = if prefix.is_empty() && suffix.is_empty() {
        None
    } else {
        Some(|variant: &Variant| {
            Ident::new(
                &format!("{}{}{}", prefix, variant.ident, suffix),
                variant.ident.span(),
            )
        })
    };

    item_enum
        .variants
        .iter()
        .filter(|variant| {
            variant
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("exclude"))
                .is_none()
        })
        .map(|variant| generate_code(&item_enum, variant, struct_name, no_impl, &enum_path))
        // Collect all of the generated structs and trait implementations into a single TokenStream
        .try_fold(quote! {}, |acc, ts| ts.map(|ts| quote! { #acc #ts }))
}

fn generate_code(
    item_enum: &ItemEnum,
    variant: &Variant,
    struct_name: Option<impl Fn(&Variant) -> Ident>,
    no_impl: bool,
    enum_path: &Path,
) -> Result<TokenStream> {
    // Generate a struct for the current variant
    let mut item_struct = generate_variant(item_enum, variant, struct_name.map(|sn| sn(variant)));

    let mut variant_attrs_iter = variant
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("variant_attrs"));
    let variant_attrs = variant_attrs_iter.next();

    if let Some(duplicate) = variant_attrs_iter.next() {
        return Err(Error::new_spanned(
            duplicate.pound_token,
            "duplicate #[variant_attrs] attribute",
        ));
    }

    // If the variant has a "variant_attrs" attribute, parse it and add the attributes to the struct
    item_struct.attrs.extend(
        variant_attrs
            .map(|attr| attr.parse_args_with(Attribute::parse_outer))
            .transpose()?
            .into_iter()
            .flatten(),
    );
    // Shortcut 1
    item_struct.attrs.extend(
        item_enum
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident("derive"))
            .cloned(),
    );
    // Shortcut 2
    item_struct.attrs.extend(
        variant
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident("doc"))
            .cloned(),
    );
    Ok(if !no_impl {
        // If the "no_impl" flag is not set, generate trait implementations for the struct
        let variant_name = Some(&variant.ident);
        let variant_impl = impl_variant(&item_struct, enum_path, variant_name);
        quote! { #item_struct #variant_impl  }
    } else {
        // Otherwise, just generate the struct without trait implementations
        quote! { #item_struct }
    })
}

impl Config {
    fn fill_prefix(&mut self, f: impl FnOnce() -> Result<Prefix>) -> Result<()> {
        match &self.prefix {
            Some(prefix) => Err(Error::new_spanned(
                prefix.0.pound_token,
                "duplicate #[prefix] attribute",
            )),
            None => Ok(self.prefix = Some(f()?)),
        }
    }
    fn fill_suffix(&mut self, f: impl FnOnce() -> Result<Suffix>) -> Result<()> {
        match &self.suffix {
            Some(suffix) => Err(Error::new_spanned(
                suffix.0.pound_token,
                "duplicate #[suffix] attribute",
            )),
            None => Ok(self.suffix = Some(f()?)),
        }
    }
    fn fill_no_impl(&mut self, f: impl FnOnce() -> Result<NoImpl>) -> Result<()> {
        match &self.no_impl {
            Some(no_impl) => Err(Error::new_spanned(
                no_impl.0.pound_token,
                "duplicate #[no_impl] attribute",
            )),
            None => Ok(self.no_impl = Some(f()?)),
        }
    }
}
