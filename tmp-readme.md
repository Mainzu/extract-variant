# extract-variant
Provide a few proc macros for extracting enum variants into their own structs.


Create top-level documentation for a Rust crate that export 2 derive macros that help extract each variant of an enum into its own structs.
The two of them are:
1. [ExtractVariant]: only applicable on enums, generate structs from every variant of the enum as well as the conversion traits ([Into] and [TryFrom]) for each of the generated structs.
2. [Variant]: only applicable on structs, requiring the user to specify what enum the struct is meant to be a variant of. Generate the same trait implementations as [ExtractVariant].

DO NOT go in-depth yet, just describe why it might be useful and provide a simple example using the following
```rust
enum Enum {
    Unit,
    Tuple(i32),
    Struct { field: f64 },
}
```

Write the documentation for a Rust derive macros `extract_variant` that extracts each variant of an enum into its own structs as well as implementing `Variant<EnumName> for VariantName`, allowing for 3 customization attributes `#[prefix(SomePrefix)]`, `#[suffix(SomeSuffix)]`, and `#[no_impl]`.
Where `Variant` is a bare trait requiring `Into` and `TryFrom`. Implementation-wise, the macro implement `From<VariantName> for EnumName`, `TryFrom<EnumName> for VariantName` (`Error = EnumName`), and `Variant<EnumName> for VariantName`.


## Usage
To use extract_variant, simply add the `#[extract_variant]` attribute to the enum that you want to extract the variants from. The variants will then be available as separate structs with the same names as the variants in the original enum.

The primary and most important one is [extract_variant]. Its simplest usage being:

```rust
use extract_variant::extract_variant;

#[extract_variant]
enum MyEnum {
    A,
    B(i32),
    C { f: f64 },
}
```
which would produce code like this:
```rust
struct A;
struct B(i32);
struct C {
    f: f64,
}
```
Note that the original enum will remain untouched.

The generated structs will have these following traits automatically implemented for them:
- [Into] the enum.
- [TryFrom] the enum to itself, the `Error` type is the enum itself.
- [Variant] of the enum. Which is basically just a requirement consisting of the previous 2 traits.

Use `no_impl` to prevent auto-implementation.
```rust
# use extract_variant::extract_variant;
#[extract_variant(no_impl)]
```

You can modify the names of the generated structs using `prefix` and `suffix` which will be prepended and appeneded to each of the generated structs respectively.

```rust
# use extract_variant::extract_variant;
#[extract_variant(prefix(MyEnum), suffix(Struct))]
```

This will instead produce `MyEnumAStruct`, `MyEnumBStruct`, and `MyEnumCStruct`.


Attributes applied to the variants will also be extracted and applied to the generated struct.

```rust
# use extract_variant::extract_variant;
#[extract_variant]
enum Value {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    Int(i64),
    #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
    Float(f64),
    #[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    Str(String),
}
```

However, for the common case of derive, and ONLY for derive, one that is placed on the enum will also be placed on each of the generated structs.
```rust
# use extract_variant::extract_variant;
#[extract_variant]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
enum Value {
    #[derive(Copy, Eq, Ord, Hash)]
    Int(i64),
    #[derive(Copy)]
    Float(f64),
    #[derive(Copy, Eq, Ord, Hash)]
    Str(String),
}
```

```rust
# use extract_variant::extract_variant;
#[extract_variant]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
enum Value {
    #[exclude_variant]
    Int(i64),
    #[derive(Copy)]
    Float(f64),
    #[derive(Copy, Eq, Ord, Hash)]
    Str(String),
}
```

In addition to being available as separate structs, the extracted variants of the enum also have several traits automatically implemented for them. These traits provide convenient ways to convert between the enum and its extracted variants, and can be useful in various contexts.

The following traits are automatically implemented for the extracted variants:

- [Into]<EnumName>: This trait allows you to convert an extracted variant into the corresponding variant of the original enum. For example, if you have an extracted variant `Variant1`, you can convert it to the `Variant1` variant of the original enum using the `Into` trait like this: `let enum_variant: EnumName = variant1.into()`.

- [TryFrom]<EnumName>: This trait allows you to try to convert a variant of the original enum into the corresponding extracted variant. If the conversion is successful, it will return the extracted variant. If the conversion fails, it will return an error of type `EnumName`. For example, if you have a variant of the original enum `Variant1`, you can try to convert it to the extracted variant `Variant1` using the `TryFrom` trait like this: `let result: Result<Variant1, EnumName> = EnumName::try_from(enum_variant)`.

- [Variant]: This trait is a combination of the `Into` and `TryFrom` traits, and requires that both of these traits are implemented for the extracted variant. It can be useful if you want to ensure that a certain type can be converted into and out of a variant of an enum in a consistent way.

Note that these traits are only implemented for the extracted variants, and not for the original enum itself. If you need to use these traits with the original enum, you will need to extract the variants from the enum and use the extracted variants instead.
