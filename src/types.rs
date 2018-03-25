/// A type.
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Type {
    Ignored,
    usize(Primitive),
    isize(Primitive),
    u8(Primitive),
    i8(Primitive),
    u16(Primitive),
    i16(Primitive),
    u32(Primitive),
    i32(Primitive),
    u64(Primitive),
    i64(Primitive),
    f32(Primitive),
    f64(Primitive),
    char(Primitive),
    bool(Primitive),
    String(Primitive),
    Struct(Struct),
    Tuple(Tuple),
    Enum(Enum),
    Opaque(Opaque),
}

impl Type {
    pub fn size(self: &Self) -> Option<usize> {
        match *self {
            Type::usize(ref p) => Some(p.size),
            Type::isize(ref p) => Some(p.size),
            Type::u8(ref p) => Some(p.size),
            Type::i8(ref p) => Some(p.size),
            Type::u16(ref p) => Some(p.size),
            Type::i16(ref p) => Some(p.size),
            Type::u32(ref p) => Some(p.size),
            Type::i32(ref p) => Some(p.size),
            Type::u64(ref p) => Some(p.size),
            Type::i64(ref p) => Some(p.size),
            Type::f32(ref p) => Some(p.size),
            Type::f64(ref p) => Some(p.size),
            Type::char(ref p) => Some(p.size),
            Type::bool(ref p) => Some(p.size),
            Type::String(ref p) => Some(p.size),
            Type::Struct(ref s) => Some(s.size),
            Type::Tuple(ref t) => Some(t.size),
            Type::Opaque(ref o) => Some(o.size),
            Type::Enum(ref o) => Some(o.size),
            _ => None
        }
    }
    pub fn name(self: &Self) -> Option<&'static str> {
        match *self {
            Type::usize(ref p) => Some(p.name),
            Type::isize(ref p) => Some(p.name),
            Type::u8(ref p) => Some(p.name),
            Type::i8(ref p) => Some(p.name),
            Type::u16(ref p) => Some(p.name),
            Type::i16(ref p) => Some(p.name),
            Type::u32(ref p) => Some(p.name),
            Type::i32(ref p) => Some(p.name),
            Type::u64(ref p) => Some(p.name),
            Type::i64(ref p) => Some(p.name),
            Type::f32(ref p) => Some(p.name),
            Type::f64(ref p) => Some(p.name),
            Type::char(ref p) => Some(p.name),
            Type::bool(ref p) => Some(p.name),
            Type::String(ref p) => Some(p.name),
            Type::Struct(ref s) => Some(s.name),
            Type::Tuple(ref t) => Some(t.name),
            Type::Opaque(ref o) => Some(o.name),
            Type::Enum(ref o) => Some(o.name),
            _ => None
        }
    }
}

/// Visibility of a type or struct member.
#[derive(Debug)]
pub enum Visibility {
    Public,
    Crate,
    Restricted,
    Inherited,
    Unknown
}

/// Field of a struct or tuple struct.
#[derive(Debug)]
pub struct Field {
    pub vis: Visibility,
    pub offset: usize,
    pub ty: Type,
    pub hints: Vec<&'static str>,
}

/// A struct (with named members).
#[derive(Debug)]
pub struct Struct {
    pub name: &'static str,
    pub size: usize,
    pub vis: Visibility,
    pub fields: Vec<(&'static str, Field)>,
}

/// A tuple struct (unnamed members).
#[derive(Debug)]
pub struct Tuple {
    pub name: &'static str,
    pub size: usize,
    pub vis: Visibility,
    pub fields: Vec<Field>,
}

/// Variant of an enum.
#[derive(Debug)]
pub struct Variant {
    pub fields: Vec<Field>,
    pub hints: Vec<&'static str>,
}

/// An enum.
#[derive(Debug)]
pub struct Enum {
    pub name: &'static str,
    pub size: usize,
    pub vis: Visibility,
    pub variants: Vec<(&'static str, Variant)>,
}

/// An opaque type.
#[derive(Debug)]
pub struct Opaque {
    pub name: &'static str,
    pub size: usize,
    pub tys: Vec<Type>,
}

/// An primitive type.
#[derive(Debug)]
pub struct Primitive {
    pub name: &'static str,
    pub size: usize,
}
