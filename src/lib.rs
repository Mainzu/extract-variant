//! Provide proc macros for extracting enum variants into their own structs.
//!
//! # Example
//! ```rust
//! use extract_variant::extract_variant;
//!
//! enum MyEnum {
//!     Variant1,
//!     Variant2(i32, String),
//!     Variant3 { field1: bool, field2: f32 },
//! }
//!
//! fn main() {
//!     let variant1 = Variant1;
//!     let variant2 = Variant2(0, String::from("hello"));
//!     let variant3 = Variant3 { field1: true, field2: 3.14 };
//! }
//! ```
//!
//! # Installation
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! extract_variant = "0.1"
//! ```
//!
//! # Usage
//! This crate provide two main attribute-like procedural macros:
//! 1. [extract_variant()] is a procedural macro that allows you to extract the variants of an enum
//! into their own structs. The extracted variants can be used as standalone structs, and have the Into,
//! TryFrom, and Variant traits automatically implemented for them. `extract_variant` can also accept a
//! prefix or suffix attribute to customize the names of the generated structs, and a variant_attrs
//! attribute to add attributes to individual variants. It can be used by adding the `#[extract_variant]`
//! attribute to an enum definition, and the extracted variants will be available as separate structs
//! with the same names as the variants in the original enum. `extract_variant` is useful for creating
//! separate types for each variant of an enum, and can make it easier to work with enums in Rust.
//!
//! 2. [variant_of()] is an attribute that allows you to create a struct that corresponds to a variant of an enum.
//! The struct can be used as a standalone type, and has the [Into][std], [TryFrom][std], and [Variant][variant_traits] traits
//! automatically implemented for it. 'variant_of' can be used by specifying the `#[variant_of]` attribute
//! on a struct definition, and specifying the name of the enum that the struct corresponds to in the
//! parentheses after the attribute. The struct must have the same fields as the variant in the enum,
//! in the same order. 'variant_of' is useful for creating a struct that corresponds to a specific variant
//! of an enum, and can make it easier to work with enums in Rust.
//!
//! In its simplest form, to use `extract_variant`, simply add the ``` attribute
//! to the enum that you want to extract the variants from. The variants will then be available
//! as separate structs with the same names as the variants in the original enum.
//!
//! Note that the original enum is not consumed by extract_variant. The original enum is still
//! available after the variants have been extracted, and can be used as normal.
//!
//! ## Custom name
//! [extract_variant()] allows you to customize the names of the generated structs by specifying
//! a prefix or suffix attribute. This can be useful if you want to avoid naming conflicts with
//! existing structs, or if you want to make the generated structs more easily distinguishable
//! from the original enum.
//!
//! To specify a prefix or suffix, add `prefix` or `suffix` to the `extract_variant`
//! attribute like this: `#[extract_variant(prefix(SomePrefix), suffix(SomeSuffix))]`.
//! They can be used together or separately.
//!
//! Note that both prefix and suffix are optional, and the generated structs will
//! have the same names as the variants in the original enum if no `prefix` or `suffix` is specified.
//!
//! ## Auto implementation
//! In addition to being available as separate structs, the extracted variants of the enum also have
//! several traits automatically implemented for them. These traits provide convenient ways to
//! convert between the enum and its extracted variants, and can be useful in various contexts.
//!
//! The following traits are automatically implemented for the extracted variants:
//! - [Into]\<EnumName\>: This trait allows you to convert an extracted variant into the corresponding
//! variant of the original enum. For example, if you have an extracted variant `Variant1`,
//! you can convert it to the `Variant1` variant of the original enum using the `Into` trait like this:
//! `let enum_variant: EnumName = variant1.into()`.
//! - [TryFrom]\<EnumName\>: This trait allows you to try to convert a variant of the original enum into
//! the corresponding extracted variant. If the conversion is successful, it will return the extracted
//! variant. If the conversion fails, it will return an error of type `EnumName`. For example,
//! if you have a variant of the original enum `Variant1`, you can try to convert it to the extracted
//! variant `Variant1` using the `TryFrom` trait like this:
//! `let result: Result<Variant1, EnumName> = EnumName::try_from(enum_variant)`.
//! - [Variant][variant_traits::Variant]\<EnumName\>: This trait is a combination of the `Into` and `TryFrom` traits, and requires
//! that both of these traits are implemented for the extracted variant. It can be useful if you want
//! to ensure that a certain type can be converted into and out of a variant of an enum in a consistent way.
//!
//! If you do not want these traits to be implemented automatically, you can specify `no_impl`
//! on the `extract_variant` attribute (`#[extract_variant(no_impl)]`).
//! This can be useful if you want to implement these traits manually
//!
//! ## Attributes
//! Attributes can be added to the generated structs by specifying the
//! `variant_attrs` attribute on the variant. This can be useful if you want to specify custom
//! behavior or metadata for the generated structs.
//!
//! To specify attributes for a variant, add the `variant_attrs` attribute to the variant like this:
//! `Variant1(i32, String) #[variant_attrs(attr1, attr2)]`. Multiple attributes can be specified
//! by separating them with commas.
//!
//! For example, if you have an enum `MyEnum` with a variant `Variant1`, and you want to add the
//! attributes `#[serde(default)]` and `#[cfg(test)]` to the generated struct, you can use the
//! following variant definition: `Variant1(i32, String) #[variant_attrs(serde(default), cfg(test))]`.
//!
//! Note that the `variant_attrs` attribute is optional, and the generated structs will not have any
//! additional attributes if no `variant_attrs` attribute is specified for the variant. However,
//! for the common case of `derive` attributes, `extract_variant` provides a convenient shortcut:
//! any `derive` attributes specified on the enum will also be applied to each of the generated structs.
//! This can save you the effort of specifying the same `derive` attributes on each variant individually.
//!
//! For example, if you have an enum `MyEnum` with a variant `Variant1`, and you want to derive
//! both `Debug` and `Clone` for the generated struct, you can use the following attribute on the
//! enum: `#[derive(Debug, Clone)]`. This will apply both the `Debug` and `Clone` derives to the
//! generated struct for `Variant1`, as well as any other variants in the enum.
//!
//! Again, this shortcut only applies to `derive` attributes, and not to other types of attributes.
//! If you want to apply other types of attributes to the generated structs, you will need to use
//! the `variant_attrs` attribute on the variant, as described above.
//!
//! ## Manually creating variants
//! In some cases, you may want to manually create a struct that corresponds to a variant of an enum,
//! without using `extract_variant` to extract the variant automatically. This can be useful if you
//! want to reuse the same usage pattern for an external enum, or if you want to create a struct that
//! corresponds to a variant of an enum that cannot be extracted by `extract_variant` (for example,
//! because it is defined in another crate or has generic parameters).
//!
//! To manually create a struct that corresponds to a variant of an enum, you can use the `#[variant_of]`
//! attribute on the struct definition. This attribute allows you to specify the enum that the struct
//! corresponds to, and will automatically implement the [Into], [TryFrom], and [Variant][variant_traits::Variant] traits for
//! the struct.
//!
//! For example, to create a struct that corresponds to the `Variant1` variant of an enum `MyEnum`,
//! you can use the following definition:
//! ```rust
//! #[variant_of(MyEnum)]
//! struct Variant1(i32, String);
//! ```
//! This will create a struct `Variant1` that corresponds to the `Variant1` variant of the `MyEnum` enum,
//! and will automatically implement the [Into], [TryFrom], and [Variant] traits for the struct.
//!
//! To use the `#[variant_of]` attribute, you must specify the name of the enum that the struct corresponds
//! to in the parentheses after the attribute. The struct must also have the same fields as the variant in
//! the enum, in the same order.
//!
//! Note that the `#[variant_of]` attribute is separate from the extract_variant procedural macro,
//! and can be used to create structs that correspond to variants of any enum, regardless of whether
//! the enum has been extracted by extract_variant or not.
//!
//! By default, variant_of assumes that the name of the struct corresponds to the name of the variant
//! in the enum, and generates the implementation accordingly. However, if you want to create a struct
//! with a different name from the variant in the enum, you can specify the name of the variant in the
//! variant_of attribute using the following syntax: #[variant_of(EnumName, VariantName)].
//!
//! For example, to create a struct named MyStruct that corresponds to the Variant1 variant of the
//! MyEnum enum, you can use the following definition:
//! ```rust
//! #[variant_of(MyEnum, Variant1)]
//! struct MyStruct(i32, String);
//! ```
//! This will create a struct MyStruct that corresponds to the Variant1 variant of the MyEnum enum,
//! and will automatically implement the Into, TryFrom, and Variant traits for the struct.
//!
//! # Limitations
//! Currently, `extract_variant` does not support extracting variants from enums with
//! generic parameters or lifetime parameters. It can only extract variants from enums
//! that are monomorphic.
//!
//! # License
//! `extract_variant` is licensed under the MIT License.
//!
//! Note: This crate's documentation is mostly generated by ChatGPT.

// mod lib;

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Attribute, Error, Fields, FieldsNamed, FieldsUnnamed, Generics,
    ItemEnum, ItemStruct, Path, PathArguments, PathSegment, Token, Variant,
};

/// A struct that holds the configuration for the [extract_variant] procedural macro.
#[derive(Debug, Default)]
struct ExtractVariant {
    /// An optional prefix to be added to the names of the generated structs.
    prefix: Option<Ident>,
    /// An optional suffix to be added to the names of the generated structs.
    suffix: Option<Ident>,
    /// A flag indicating whether the [Into], [TryFrom], and [Variant][variant_traits::Variant] traits should be implemented automatically for the generated structs.
    no_impl: bool,
}

/// A struct that holds the configuration for the [variant_of] attribute.
struct VariantOf {
    /// The path to the enum that the struct corresponds to.
    enum_path: Path,
    /// The name of the variant in the enum that the struct corresponds to.
    variant_name: Option<Ident>,
}

/// A struct that holds the attributes to be applied to a variant when it is extracted by the [extract_variant] procedural macro.
struct VariantAttrs(Vec<Attribute>);

#[proc_macro_attribute]
pub fn extract_variant(attr_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as ItemEnum);
    // If the ItemEnum has generic parameters, return a compile-time error
    if let Some(lt_token) = item_enum.generics.lt_token {
        return Error::new(
            lt_token.span,
            "`extract_variant` does not support generic parameters",
        )
        .to_compile_error()
        .into();
    }

    let ExtractVariant {
        prefix,
        suffix,
        no_impl,
    } = parse_macro_input!(attr_args);

    // Convert the prefix and suffix Ident values to strings, if they are present
    let prefix = prefix.map(|id| id.to_string()).unwrap_or_default();
    let suffix = suffix.map(|id| id.to_string()).unwrap_or_default();

    // Keep track of the path to the enum to generate conversion impls
    // AI: Create a path to the enum using its identifier
    //
    // The reason it's a path and not just an identifier is because the function that generate the
    // implementations is general. This is to support `variant_of`.
    let enum_path = Path::from(PathSegment {
        ident: item_enum.ident.clone(),
        arguments: PathArguments::None,
    });

    // Create a closure to generate modified variant names if prefix or suffix is non-empty
    let variant_name = if prefix == "" && suffix == "" {
        None
    } else {
        Some(|variant: &Variant| format!("{}{}{}", prefix, variant.ident, suffix))
    };

    // Iterate over the variants in the enum
    let tss = item_enum.variants.iter().map(|variant| {
        let name = format!("{}{}{}", prefix, variant.ident, suffix);
        // Generate a struct for the current variant
        let mut strct = generate_variant(&name, &item_enum, variant);
        // If the variant has a "variant_attrs" attribute, parse it and add the attributes to the struct
        if let Some(res) = variant
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("variant_attrs"))
            .map(Attribute::parse_args::<VariantAttrs>)
        {
            match res {
                Err(err) => return err.into_compile_error().into(),
                Ok(VariantAttrs(attrs)) => strct.attrs.extend(attrs),
            }
        }
        // If the "no_impl" flag is not set, generate trait implementations for the struct
        if no_impl == false {
            let variant_name = variant_name
                .map(|vn| vn(variant))
                .map(|s| Ident::new(s.as_str(), variant.ident.span()));
            let variant_impl = impl_variant(&strct, &enum_path, variant_name);
            quote! { #strct #variant_impl  }
        } else {
            // Otherwise, just generate the struct without trait implementations
            quote! { #strct }
        }
    });

    // Collect all of the generated structs and trait implementations into a single TokenStream
    let init = quote! { #item_enum };
    TokenStream::from(tss.fold(init, |acc, ts| quote! { #acc #ts }))
}

#[proc_macro_attribute]
pub fn variant_of(attr_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let VariantOf {
        enum_path,
        variant_name,
    } = parse_macro_input!(attr_args as VariantOf);

    let strct = &item_struct;

    TokenStream::from(impl_variant(strct, &enum_path, variant_name))
}

// #[proc_macro_attribute]
// pub fn variant_attrs(_: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }

/// Generates a struct definition from a variant of an enum.
///
/// # Parameters
///
/// - `name`: A string slice representing the name of the struct to be generated.
/// - `item_enum`: A reference to an `ItemEnum` struct representing the original enum.
/// - `variant`: A reference to a `Variant` struct representing the variant to be extracted.
///
/// # Returns
///
/// An `ItemStruct` struct representing the generated struct definition.
fn generate_variant(name: &str, item_enum: &ItemEnum, variant: &Variant) -> ItemStruct {
    ItemStruct {
        attrs: variant.attrs.clone(),
        vis: item_enum.vis.clone(),
        struct_token: token::Struct(variant.ident.span()),
        ident: Ident::new(name, variant.ident.span()),
        generics: Generics::default(),
        fields: variant.fields.clone(),
        semi_token: None,
    }
}

/// Generates a block of code that implements the `Into`, `TryFrom`, and `Variant` traits for a struct that corresponds to a variant of an enum.
///
/// # Parameters
///
/// - `strct`: A reference to an `ItemStruct` struct representing the struct for which the traits should be implemented.
/// - `enum_path`: A reference to a `Path` struct representing the path to the enum that the struct corresponds to.
/// - `variant_name`: An optional `Ident` struct representing the name of the variant in the enum that the struct corresponds to. If this parameter is `None`, the function will use the name of the struct as the name of the variant.
///
/// # Returns
///
/// A `TokenStream` representing the generated block of code.
fn impl_variant(
    strct: &ItemStruct,
    enum_path: &Path,
    variant_name: Option<Ident>,
) -> proc_macro2::TokenStream {
    // Make it usable in a quote! block
    let strct_ident = &strct.ident;
    let variant_name = variant_name.as_ref().unwrap_or(&strct.ident);

    let (free, tied) = match &strct.fields {
        // If the fields are named, bind the names to variables and use them to create the trait implementations
        Fields::Named(FieldsNamed { named, .. }) => {
            let names: Vec<&Ident> = named
                .into_iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect();
            (
                quote! { #strct_ident { #(#names),* } },
                quote! { #variant_name { #(#names),* } },
            )
        }
        // If the fields are unnamed, bind the fields to variables with names like "_0", "_1", etc. and use them to create the trait implementations
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let id: Vec<Ident> = (0..unnamed.len())
                .map(|i| Ident::new(&format!("_{}", i), Span::call_site()))
                .collect();
            (
                quote! { #strct_ident(#(#id),*) },
                quote! { #variant_name(#(#id),*) },
            )
        }
        // If the fields are a unit type, bind no variables and use them to create the trait implementations
        Fields::Unit => (quote! { #strct_ident }, quote! { #variant_name }),
    };

    // Create the `From` and `TryFrom` trait implementations
    let froms = quote! {
        impl ::std::convert::From<#strct_ident> for #enum_path {
            fn from(#free: #strct_ident) -> Self {
                Self::#tied
            }
        }
        impl ::std::convert::TryFrom<#enum_path> for #strct_ident {
            type Error = #enum_path;
            fn try_from(value: #enum_path) -> ::std::result::Result<Self, Self::Error> {
                if let #enum_path::#tied = value { Ok(#free) } else { Err(value) }
            }
        }
    };

    // Return a `TokenStream` containing the trait implementations
    quote! {
        #froms
        impl ::variant_traits::Variant<#enum_path> for #strct_ident {}
    }
}

// ================================================================================================
// ------------------------------------------------------------------------------------------------
// ================================================================================================

impl Parse for ExtractVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut extract_variant = Self::default();

        if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);

            while !content.is_empty() {
                let ident: Ident = content.parse()?;
                match ident.to_string().as_ref() {
                    "prefix" => {
                        let inner_content;
                        parenthesized!(inner_content in content);
                        extract_variant.prefix = Some(inner_content.parse()?)
                    }
                    "suffix" => {
                        let inner_content;
                        parenthesized!(inner_content in content);
                        extract_variant.suffix = Some(inner_content.parse()?)
                    }
                    "no_impl" => extract_variant.no_impl = true,
                    _ => return Err(syn::Error::new(ident.span(), "invalid parameter name")),
                }
            }
        }

        Ok(extract_variant)
    }
}

impl Parse for VariantAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(Self(content.call(Attribute::parse_outer)?))
    }
}
impl Parse for VariantOf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let enum_path = content.parse()?;
        content.parse::<Token![,]>()?;
        let variant_name = content.parse()?;
        Ok(Self {
            enum_path,
            variant_name,
        })
    }
}
