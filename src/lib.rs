/*!
 * Run-time type information trait. Use crate rtti-derive to implement.
 *
 * **very early, probably best to stay away for now**
 */

/// Provides run-time type information.
pub trait RTTI {
    /// Returns a Type enum describing the type.
    fn rtti() -> Type;
}

// implement built in types

macro_rules! implement_rtti {
    ($t:ty, $i:ident) => (
        impl $crate::RTTI for $t {
            #[inline(always)]
            fn rtti() -> $crate::Type {
                $crate::Type::$i
            }
        }
    )
}

implement_rtti!(usize, usize);
implement_rtti!(isize, isize);
implement_rtti!(u8, u8);
implement_rtti!(i8, i8);
implement_rtti!(u16, u16);
implement_rtti!(i16, i16);
implement_rtti!(u32, u32);
implement_rtti!(i32, i32);
implement_rtti!(u64, u64);
implement_rtti!(i64, i64);
implement_rtti!(f32, f32);
implement_rtti!(f64, f64);
implement_rtti!(char, char);
implement_rtti!(bool, bool);

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