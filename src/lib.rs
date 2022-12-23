#![feature(rustdoc_missing_doc_code_examples)]
#![deny(rustdoc::missing_doc_code_examples)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Fields, FieldsNamed, FieldsUnnamed, Generics, ItemEnum, ItemStruct,
    Path, Token, Variant, VisPublic, Visibility,
};

// mod lib;
mod extract_variant;
mod variant_of;

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
    variant_ident: Option<Ident>,
}

/// Extracts each variant in an enum into its own standalone struct, then implements conversion traits
/// between the original enum and the generated struct.
///
/// # Example
/// ```rust, no_run
/// # use extract_variant::extract_variant;
/// #[derive(extract_variant)]
/// enum MyEnum {
///     UnitVariant,
///     TupleVariant(i32),
///     StructVariant { field: f64, },
/// }
/// fn main() {
///     let unit_variant = UnitVariant;
///     let tuple_variant = TupleVariant(42);
///     let struct_variant = StructVariant { field: 3.14 };
/// }
/// ```
///
/// # Auto implementations
/// In addition to being available as separate structs, the extracted variants of the enum also have
/// several traits automatically implemented for them. These traits provide convenient ways to
/// convert between the enum and its extracted variants.
///
/// For each variant, `V`, extracted from an enum, `E`, traits [`Into<E>`], [`TryFrom<E, Error = E>`],
/// and [`Variant<E>`][variant_traits::Variant] are automatically implemented for `V`.
/// Where [`Variant<E>`][variant_traits::Variant] is just a bare trait that requires the previous two
/// traits to be implemented. It can be useful if you want to ensure that a certain type can be
/// converted into and from an enum in a consistent way.
///
/// This behavior can be disabled when desired using the `#[no_impl]` attribute.
/// ```rust, no_run
/// # use extract_variant::extract_variant;
/// #[derive(extract_variant)]
/// #[no_impl]
/// enum MyEnum {
///     UnitVariant,
/// }
/// # fn main() {
/// #   let unit_variant = UnitVariant;
/// # }
/// ```
/// ```rust, compile_fail
/// # use extract_variant::extract_variant;
/// # #[derive(extract_variant)]
/// # #[no_impl]
/// # enum MyEnum {
/// #     UnitVariant,
/// # }
/// # fn main() {
/// let my_enum = MyEnum::from(UnitVariant); // fails to compile
/// # }
/// ```
/// ```rust, compile_fail
/// # use extract_variant::extract_variant;
/// # use extract_variant::extract_variant;
/// # #[derive(extract_variant)]
/// # #[no_impl]
/// # enum MyEnum {
/// #     UnitVariant,
/// # }
/// # fn main() {
/// let unit_variant = UnitVariant::try_from(MyEnum::UnitVariant); // also fails to compile
/// # }
/// ```
///
/// # Name customization
/// The names of the generated structs can be customized by specifying a `#[prefix(...)]`
/// or `#[suffix(...)]` attribute. This can be useful if you want to avoid naming conflicts
/// with existing structs or if you want to make the generated structs more easily distinguishable.
/// They can be used together or separately.
///
/// ```rust, no_run
/// # use extract_variant::extract_variant;
/// #[derive(extract_variant)]
/// #[prefix(MyEnum)]
/// enum MyEnum {
///     UnitVariant,
///     TupleVariant(i32),
///     StructVariant { field: f64, },
/// }
/// fn main() {
///     let unit_variant = MyEnumUnitVariant;
///     let tuple_variant = MyEnumTupleVariant(42);
///     let struct_variant = MyEnumStructVariant { field: 3.14 };
/// }
/// ```
///
/// Note that both the prefix and suffix are optional, and the generated structs will
/// have the same names as the variants in the original enum if no `prefix` or `suffix` is specified.
///
/// # Attributes
/// Attributes can be added to the generated structs by specifying the #[variant_attrs(...)] attribute
/// on the desired variant. Apart from this, however, for the common cases of `derive` and `doc`,
/// a convenient shortcut exists. Firstly, `derive` attributes specified on the enum will be inherited
/// by all extracted variants. **Important note.[^important note]**
/// This can save you the effort of specifying the same `derive` attributes on each variant individually.
/// Secondly, the doc comments of a variant are also passed on to the generated struct.
///
/// ```rust, no_run
/// # use extract_variant::extract_variant;
/// #[derive(extract_variant, Default)]
/// // The variants won't derive `Default` since it's in the same block as `extract_variant`
/// // This comment also serves to prevent a formatter from merging the two `derive` blocks
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// enum MyEnum {
///     #[default]
///     /// A variant of [MyEnum]
///     UnitVariant,
///     /// A variant of [MyEnum]
///     #[variant_attrs(
///         #[derive(Eq, PartialOrd, Ord)] // `TupleVariant`'s own derive
///     )]
///     TupleVariant(i32),
///     /// A variant of [MyEnum]
///     StructVariant { field: f64, },
/// }
/// fn main() {
///     println!("{:?}", UnitVariant);
///     println!("{:?}", TupleVariant(42));
///     println!("{:?}", StructVariant { field: 3.14, });
/// }
/// ```
///
/// [^important note]: Any other traits placed within the same derive block as the `extract_variant`
/// will NOT be inheritted by the generated structs. Be sure to place the traits you want derived
/// by the generated structs in another block. The derive block with `extract_variant` will only apply
/// to the enum.
/// I did not explicitly implemented this behavior. I am also not knowledgable enough to tell whether is
/// a garuanteed behavior or not. If the describe behavior no longer applies in the future,
/// you can assume that it was not garuanteed.
///
/// # Limitations
/// Currently, `extract_variant` does not support extracting variants from enums with
/// generic parameters or lifetime parameters. It can only extract variants from enums
/// that are monomorphic.
///
/// # TODO
/// Document `#[exclude]` attribute. Just put it on any variant that shouldn't be extracted.
///
#[proc_macro_derive(
    extract_variant,
    attributes(prefix, suffix, no_impl, variant_attrs, exclude)
)]
pub fn extract_variant(input: TokenStream) -> TokenStream {
    match extract_variant::doit(parse_macro_input!(input)) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// TODO
///
/// In some cases, you may want to manually create a struct that corresponds to a variant of an enum,
/// without using [`extract_variant`][extract_variant()] to extract the variant automatically.
/// This can be useful if you want to create a struct that corresponds to a variant of an enum
/// that cannot be extracted by [`extract_variant`][extract_variant()] (for example,
/// because it is defined in another crate or has generic parameters).
///
/// # Example
/// ```rust, no_run
/// use extract_variant::Variant;
///
/// mod my_mod {
///     pub enum MyEnum {
///         UnitVariant,
///         TupleVariant(i32),
///         StructVariant { field: f64, },
///     }   
/// }
///
/// #[derive(Variant)]
/// #[variant_of(my_mod::MyEnum)]
/// struct UnitVariant;
///
/// #[derive(Variant)]
/// #[variant_of(my_mod::MyEnum)]
/// struct TupleVariant(i32);
///
/// #[derive(Variant)]
/// #[variant_of(my_mod::MyEnum)]
/// struct StructVariant {
///     field: f64,
/// }
/// ```
#[proc_macro_derive(Variant, attributes(variant_of))]
pub fn derive_variant(input: TokenStream) -> TokenStream {
    match variant_of::doit(parse_macro_input!(input)) {
        Ok(token_stream) => token_stream.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

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
fn generate_variant(
    item_enum: &ItemEnum,
    variant: &Variant,
    struct_name: Option<Ident>,
) -> ItemStruct {
    let mut fields = variant.fields.clone();
    for field in &mut fields {
        field.vis = Visibility::Public(VisPublic {
            pub_token: token::Pub::default(),
        });
    }
    ItemStruct {
        attrs: Vec::new(),
        vis: item_enum.vis.clone(),
        struct_token: token::Struct(variant.ident.span()),
        ident: struct_name.unwrap_or_else(|| variant.ident.clone()),
        generics: Generics::default(),
        fields,
        semi_token: None,
    }
}

/// Generates a block of code that implements the `Into`, `TryFrom`, and `Variant` traits for a struct that corresponds to a variant of an enum.
///
/// # Parameters
///
/// - `item_struct`: A reference to an `ItemStruct` struct representing the struct for which the traits should be implemented.
/// - `enum_path`: A reference to a `Path` struct representing the path to the enum that the struct corresponds to.
/// - `variant_name`: An optional `Ident` struct representing the name of the variant in the enum that the struct corresponds to. If this parameter is `None`, the function will use the name of the struct as the name of the variant.
///
/// # Returns
///
/// A `TokenStream` representing the generated block of code.
fn impl_variant(
    item_struct: &ItemStruct,
    enum_path: &Path,
    variant_ident: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let struct_path = Path::from(item_struct.ident.clone());
    let variant_ident = variant_ident.unwrap_or(&item_struct.ident);

    // Create the `From` and `TryFrom` trait implementations
    let froms = impl_froms(
        &struct_path,
        enum_path,
        variant_ident,
        fields_stream(&item_struct.fields),
    );

    // Return a `TokenStream` containing the trait implementations
    quote! {
        #froms
        impl ::variant_traits::Variant<#enum_path> for #struct_path {}
    }
}

fn fields_stream(fields: &Fields) -> TokenStream2 {
    match fields {
        // If the fields are named, bind the names to variables and use them to create the trait implementations
        Fields::Named(FieldsNamed { named, .. }) => {
            let names = named.into_iter().map(|f| f.ident.as_ref().unwrap());
            quote! { { #(#names),* } }
        }
        // If the fields are unnamed, bind the fields to variables with names like "_0", "_1", etc. and use them to create the trait implementations
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let id = (0..unnamed.len()).map(|i| Ident::new(&format!("_{}", i), Span::call_site()));
            quote! { (#(#id),*) }
        }
        // If the fields are a unit type, bind no variables and use them to create the trait implementations
        Fields::Unit => quote! {},
    }
}
fn impl_froms(
    struct_path: &Path,
    enum_path: &Path,
    variant_ident: &Ident,
    fields_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        impl ::std::convert::From<#struct_path> for #enum_path {
            fn from(#struct_path #fields_stream: #struct_path) -> Self {
                Self::#variant_ident #fields_stream
            }
        }
        impl ::std::convert::TryFrom<#enum_path> for #struct_path {
            type Error = #enum_path;
            fn try_from(value: #enum_path) -> ::std::result::Result<Self, Self::Error> {
                if let #enum_path::#variant_ident #fields_stream = value { Ok(#struct_path #fields_stream) } else { Err(value) }
            }
        }
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

impl Parse for VariantOf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_path = input.parse()?;
        let variant_ident = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self {
            enum_path,
            variant_ident,
        })
    }
}
