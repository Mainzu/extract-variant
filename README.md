Provide proc macros for extracting enum variants into their own structs.


Can be useful for enhancing Rust's already-robust type system.


# Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
extract_variant = "0.1"
```


Currently, two derive macros are available:
1. [`extract_variant`][extract_variant()]: This is the one you're looking for; it automatically "extracts" the variants of an enum into their own standalone structs and implements conversion-related traits between the enum and the generated struct. In this case, extraction does not consume the enum and instead leaves it alone.
2. [`Variant`][derive_variant]: In case you've already got an enum but can't change it, derive this on a struct to automatically create implementations of [`Into<Enum>`], [`TryFrom<Enum>`], and [`Variant<Enum>`][variant_traits::Variant] (Where ['VariantEnum>'][variant_traits::Variant] is just a bare trait that requires the first two).The fields must be identical to the variant you're aiming for in name, type, and order (if any exist).


# Example
```rust, no_run
use extract_variant::extract_variant;


#[derive(extract_variant)]
enum MyEnum {
    UnitVariant,
    TupleVariant(i32),
    StructVariant { field: f64, },
}
```
would generate the equivalent:
```rust, no_run
struct UnitVariant;
struct TupleVariant(i32);
struct StructVariant { field: f64, }
```
# Motivation
When I know an enum value must be a specific variant in order for my code to function, I have two options. Either, I just [panic!] when the value is not the expected variant (which is unideal since it doesn't allow for any further error handling), create a whole new type that is identical to the expected variant and receive that specific type (in the case of a function), or implement and use [TryFrom::try_from] which, when I have to do it for every variant, can be a very dull and repetitive task.