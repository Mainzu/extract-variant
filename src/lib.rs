// extern crate proc_macro;
// extern crate proc_macro2;
// extern crate quote;
// extern crate syn;

// use proc_macro::TokenStream;
// use proc_macro2::{Ident, Span};
// use quote::quote;
// use syn::{
//     parse::Parse, parse_macro_input, parse_quote, token, Attribute, Fields, FieldsNamed,
//     FieldsUnnamed, Generics, ItemEnum, ItemStruct, Path, PathArguments, PathSegment, Variant,
//     Visibility,
// };

// fn get_attr_as<'a, T: Parse>(
//     mut attrs: impl Iterator<Item = &'a Attribute>,
//     path: &str,
// ) -> Option<T> {
//     attrs
//         .find(|attr| attr.path.is_ident(path))
//         .map(Attribute::parse_args::<T>)
//         .and_then(Result::ok)
// }

// #[proc_macro_derive(Variant, attributes(variation_prefix, variation_suffix))]
// pub fn derive_variant(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as ItemEnum);

//     let ItemEnum {
//         attrs,
//         vis,
//         enum_token: _,
//         ident,
//         generics,
//         brace_token: _,
//         variants,
//     } = ast;

//     if !generics.params.is_empty() {
//         panic!("Generic parameter not supported")
//     }

//     let prefix = get_attr_as::<Ident>(attrs.iter(), "variation_prefix")
//         .map(|id| id.to_string())
//         .unwrap_or_else(String::default);
//     let suffix = get_attr_as::<Ident>(attrs.iter(), "variation_suffix")
//         .map(|id| id.to_string())
//         .unwrap_or_else(String::default);

//     let strcts = variants.into_iter().map(|variant| {
//         let variant_ident = variant.ident.clone();
//         variant_to_struct(
//             variant,
//             Path::from(PathSegment {
//                 ident: ident.clone(),
//                 arguments: PathArguments::None,
//             }),
//             Ident::new(
//                 &format!("{}{}{}", prefix, variant_ident, suffix),
//                 variant_ident.span(),
//             ),
//             vis.clone(),
//         )
//     });

//     TokenStream::from(strcts.fold(
//         quote! { impl ::variant_trait::Variant for #ident {} },
//         |acc, strct| quote! { #acc #strct },
//     ))
// }

// fn variant_to_struct(
//     Variant {
//         mut attrs,
//         ident,
//         fields,
//         discriminant: _,
//     }: Variant,
//     variant: Path,
//     vv: Ident,
//     vis: Visibility,
// ) -> ItemStruct {
//     attrs.push(parse_quote! { #[derive(Variation)] });
//     attrs.push(parse_quote! { #[variation_of(#variant)] });
//     attrs.push(parse_quote! { #[variant_name(#ident)] });
//     ItemStruct {
//         attrs,
//         vis,
//         struct_token: token::Struct(Span::call_site()),
//         ident: vv,
//         generics: Generics::default(),
//         fields,
//         semi_token: None,
//     }
// }

// fn impl_variation(
//     ItemStruct { ident, fields, .. }: &ItemStruct,
//     variant: Path,
//     vv: Ident,
// ) -> TokenStream {
//     let (free, tied) = match fields {
//         Fields::Named(FieldsNamed { named, .. }) => {
//             let names: Vec<&Ident> = named
//                 .into_iter()
//                 .map(|f| f.ident.as_ref().unwrap())
//                 .collect();
//             (
//                 quote! { #ident { #(#names),* } },
//                 quote! { #vv { #(#names),* } },
//             )
//         }
//         Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//             let id: Vec<Ident> = (0..unnamed.len())
//                 .map(|i| Ident::new(&format!("_{}", i), Span::call_site()))
//                 .collect();
//             (quote! { #ident(#(#id),*) }, quote! { #vv(#(#id),*) })
//         }
//         Fields::Unit => (quote! { #ident }, quote! { #vv }),
//     };

//     let froms = quote! {
//         impl ::std::convert::From<#ident> for #variant {
//             fn from(#free: #ident) -> Self {
//                 Self::#tied
//             }
//         }
//         impl ::std::convert::TryFrom<#variant> for #ident {
//             type Error = #variant;
//             fn try_from(value: #variant) -> Result<Self, Self::Error> {
//                 if let #variant::#tied = value { Ok(#free) } else { Err(value) }
//             }
//         }
//     };

//     TokenStream::from(quote! {
//         #froms
//         impl ::variant_trait::Variation<#variant> for #ident {}
//     })
// }

// #[proc_macro_derive(Variation, attributes(variation_of, variant_name))]
// pub fn derive_variation(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as ItemStruct);

//     let variant = get_attr_as::<Path>(ast.attrs.iter(), "variation_of")
//         .expect("require `variation_of` attribute (#[variation_of(EnumPath)])");
//     let vv =
//         get_attr_as::<Ident>(ast.attrs.iter(), "variant_name").unwrap_or_else(|| ast.ident.clone());

//     impl_variation(&ast, variant, vv)
// }
