use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeContext {
    types: HashMap<String, Type>,
}

impl TypeContext {
    pub fn new() -> TypeContext {
        TypeContext {
            types: HashMap::new(),
        }
    }

    /// Returns a type description for the specified type.
    ///
    /// If T is a basic type, this will return its concrete type description. Otherwise, an id
    /// will be returned.
    pub fn type_or_id_for<T: Reflect>(&mut self) -> Type {
        if T::__is_basic_type() {
            return T::type_description(self);
        }

        let id = T::type_id();

        if !self.types.contains_key(&id) {
            let type_desc = T::type_description(self);
            self.types.insert(id.clone(), type_desc);
        }

        Type::Id(id)
    }

    /// Returns the inner map storing type descriptions.
    ///
    /// This is a map from type id to the corresponding description.
    pub fn into_types(self) -> HashMap<String, Type> {
        self.types
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Reflect {
    fn type_id() -> String;
    fn type_description(cx: &mut TypeContext) -> Type;

    #[doc(hidden)]
    fn __is_basic_type() -> bool {
        false
    }
}

#[derive(Debug)]
pub enum Type {
    Basic(BasicType),
    Struct(StructType),
    Enum(EnumType),
    Id(String),
}

#[derive(Debug)]
pub enum BasicType {
    Primitive(PrimitiveType),
    Option(Box<Type>),
    Array(Box<Type>),
    Map(Box<Type>),
}

#[derive(Clone, Copy, Debug)]
pub enum IntWidth {
    W8 = 8,
    W16 = 16,
    W32 = 32,
    W64 = 64,
    W128 = 128,
}

impl IntWidth {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum FloatWidth {
    F32 = 32,
    F64 = 64,
}

impl FloatWidth {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Debug)]
pub enum PrimitiveType {
    Bool,
    Int(IntWidth),
    UInt(IntWidth),
    Float(FloatWidth),
    String,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub flatten: bool,
    pub type_desc: Type,
}

#[derive(Debug)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub enum EnumTag {
    Internal(String),
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub tag_value: String,
    pub kind: EnumVariantKind,
}

#[derive(Debug)]
pub enum EnumVariantKind {
    Unit,
    NewType(Type),
    Struct(Vec<Field>),
}

#[derive(Debug)]
pub struct EnumType {
    pub name: String,
    pub tag: Option<EnumTag>,
    pub variants: Vec<EnumVariant>,
}

mod impls {
    use super::*;

    macro_rules! impl_for_primitive {
        ($t:ty, $e:expr) => {
            impl Reflect for $t {
                fn type_id() -> String {
                    stringify!($t).to_owned()
                }

                fn type_description(_: &mut TypeContext) -> Type {
                    Type::Basic(BasicType::Primitive($e))
                }

                fn __is_basic_type() -> bool {
                    true
                }
            }
        };
    }

    impl_for_primitive!(i8, PrimitiveType::Int(IntWidth::W8));
    impl_for_primitive!(i16, PrimitiveType::Int(IntWidth::W16));
    impl_for_primitive!(i32, PrimitiveType::Int(IntWidth::W32));
    impl_for_primitive!(i64, PrimitiveType::Int(IntWidth::W64));
    impl_for_primitive!(i128, PrimitiveType::Int(IntWidth::W128));

    impl_for_primitive!(u8, PrimitiveType::UInt(IntWidth::W8));
    impl_for_primitive!(u16, PrimitiveType::UInt(IntWidth::W16));
    impl_for_primitive!(u32, PrimitiveType::UInt(IntWidth::W32));
    impl_for_primitive!(u64, PrimitiveType::UInt(IntWidth::W64));
    impl_for_primitive!(u128, PrimitiveType::UInt(IntWidth::W128));

    impl_for_primitive!(f32, PrimitiveType::Float(FloatWidth::F32));
    impl_for_primitive!(f64, PrimitiveType::Float(FloatWidth::F64));

    impl_for_primitive!(bool, PrimitiveType::Bool);
    impl_for_primitive!(String, PrimitiveType::String);

    impl<T: Reflect> Reflect for Option<T> {
        fn type_id() -> String {
            format!("Option<{}>", T::type_id())
        }

        fn type_description(cx: &mut TypeContext) -> Type {
            Type::Basic(BasicType::Option(Box::new(cx.type_or_id_for::<T>())))
        }

        fn __is_basic_type() -> bool {
            true
        }
    }

    impl<T: Reflect> Reflect for Vec<T> {
        fn type_id() -> String {
            format!("Vec<{}>", T::type_id())
        }

        fn type_description(cx: &mut TypeContext) -> Type {
            Type::Basic(BasicType::Array(Box::new(cx.type_or_id_for::<T>())))
        }

        fn __is_basic_type() -> bool {
            true
        }
    }
}
