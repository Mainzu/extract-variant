// use extract_variant::{extract_variant, Variant};

// #[derive(extract_variant)]
// // #[no_impl]
// // #[prefix(MyEnum)]
// #[derive(Debug)]
// enum MyEnum {
//     #[variant_attrs(
//         #[derive(Default)]
//     )]
//     Variant1,
//     Variant2(i32, String),
//     Variant3 {
//         field1: bool,
//         field2: f32,
//     },
// }

// enum Hi {
//     Ayo,
// }

// #[derive(Variant)]
// #[variant_of(Hi, Ayo)]
// struct Ayo;

// fn main() {
//     println!("{:?}", Variant1);
//     let variant1 = Variant1;
//     let variant2 = Variant2(0, String::from("hello"));
//     let variant3 = Variant3 {
//         field1: true,
//         field2: 3.14,
//     };
// }

// // #[derive(Debug, Clone, PartialEq, Default)]
// // enum Value {
// //     #[default]
// //     #[variant_attrs(
// //         #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// //     )]
// //     Undefined,
// //     #[variant_attrs(
// //         #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// //     )]
// //     Bool(bool),
// //     #[variant_attrs(
// //         #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
// //     )]
// //     Number(f64),
// //     #[variant_attrs(#[
// //         derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// //     )]
// //     String(std::string::String),
// //     #[variant_attrs(
// //         #[derive(Debug, Clone, Default, PartialEq)]
// //     )]
// //     Array(Vec<Value>),
// //     #[variant_attrs(
// //         #[derive(Debug, Clone, Default, PartialEq)]
// //     )]
// //     Object(BTreeMap<Value, Value>),
// // }

// // #[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
// // enum Value {
// //     #[default]
// //     Undefined,
// //     Bool(bool),
// //     Number(f64),
// //     String(std::string::String),
// //     Array(Vec<Value>),
// //     Object(BTreeMap<Value, Value>),
// // }

// // #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// // struct Undefined;

// // #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// // struct Bool(bool);
// // #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
// // struct Number(f64);
// // #[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
// // struct String(std::string::String);
// // #[derive(Debug, Clone, Default, PartialEq)]
// // struct Array(Vec<Value>);
// // #[derive(Debug, Clone, Default, PartialEq)]
// // struct Object(BTreeMap<Value, Value>);
