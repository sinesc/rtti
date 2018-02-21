/// A type.
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Type {
    Ignored,
    usize,
    isize,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    f32,
    f64,
    char,
    bool,
    Struct(Struct),
    Tuple(Tuple),
    Opaque(Opaque),
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
    pub ty: Box<Type>,
}

/// A struct (with named members).
#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub vis: Visibility,
    pub fields: Vec<(String, Field)>,
}

/// A tuple struct (unnamed members).
#[derive(Debug)]
pub struct Tuple {
    pub name: String,
    pub vis: Visibility,
    pub fields: Vec<Field>,
}

/// An opaque type.
#[derive(Debug)]
pub struct Opaque {
    pub name: String,
    pub tys: Vec<Box<Type>>,
}
