pub trait Variant<Enum>: Into<Enum> + TryFrom<Enum, Error = Enum> {}
