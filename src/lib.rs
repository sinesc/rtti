/*!
Run-time type information trait. Use crate [`rtti-derive`](https://crates.io/crates/rtti-derive) to implement.

To include RTTI, use:

```
#[macro_use]
extern crate rtti_derive;
extern crate rtti;
use rtti::RTTI;
# fn main() {}
```

You can then implement `rtti()` for a custom type:

```
# #[macro_use]
# extern crate rtti_derive;
# extern crate rtti;
# use rtti::RTTI;
#
#[derive(RTTI)]
struct Simple {
    x: u32,
    pub y: ::std::sync::Arc<u32>,
    pub(crate) z: Vec<f64>
}

fn main() {
    println!("{:?}", Simple::ctti());
}
```

You can ignore fields or add hints using the ignore and hint attributes:

```
# #[macro_use]
# extern crate rtti_derive;
# extern crate rtti;
# use rtti::RTTI;
#
struct UnsupportedForeignType ();

#[derive(RTTI)]
struct Attributed {
    #[rtti(hint = "foo")]
    #[rtti(hint = "bar")]
    pub foobard: ::std::sync::Arc<u32>,
    #[rtti(ignore)]
    #[rtti(hint = "sets type to Type::Ignored")]
    ignored: UnsupportedForeignType,
}

fn main() {
    println!("{:?}", Attributed::ctti());
}
```

When implementing RTTI for a generic type, make sure generic parameters implement RTTI:

```
# #[macro_use]
# extern crate rtti_derive;
# extern crate rtti;
# use rtti::RTTI;
#[derive(RTTI)]
struct Generic<T> where T: RTTI {
    test: T,
    stuff: i32,
}

fn main() {
    println!("{:?}", Generic::<u64>::ctti());
}
```
*/
#[macro_use]
mod macros;
mod types;
pub use types::*;

/// Provides run-time type information.
pub trait RTTI {
    /// Returns a Type enum describing the type.
    fn ctti() -> Type;
    /// Returns a Type enum describing the type of the instance.
    fn rtti(self: &Self) -> Type {
        Self::ctti()
    }
}

#[doc(hidden)]
pub struct Ignored ();

impl RTTI for Ignored {
    fn ctti() -> Type {
        Type::Ignored
    }
}

// implement built in types
mod wrapper {
    impl_prim!(usize, usize);
    impl_prim!(isize, isize);
    impl_prim!(u8, u8);
    impl_prim!(i8, i8);
    impl_prim!(u16, u16);
    impl_prim!(i16, i16);
    impl_prim!(u32, u32);
    impl_prim!(i32, i32);
    impl_prim!(u64, u64);
    impl_prim!(i64, i64);
    impl_prim!(f32, f32);
    impl_prim!(f64, f64);
    impl_prim!(char, char);
    impl_prim!(bool, bool);
    impl_prim!(String, String);
    impl_opaque!(::std::rc::Rc, Rc);
    impl_opaque!(::std::cell::Cell, Cell);
    impl_opaque!(::std::cell::RefCell, RefCell);
    impl_opaque!(::std::cell::UnsafeCell, UnsafeCell);
    impl_opaque!(::std::option::Option, Option);
    impl_opaque!(::std::boxed::Box, Box);
    impl_opaque!(::std::sync::Arc, Arc);
    impl_opaque!(::std::sync::Mutex, Mutex);
    impl_opaque!(::std::sync::RwLock, RwLock);
    impl_opaque!(::std::sync::Weak, Weak);
    impl_opaque!(::std::vec::Vec, Vec);
    impl_opaque!(::std::collections::VecDeque, VecDeque);
    impl_opaque!(::std::collections::LinkedList, LinkedList);
    impl_opaque!(::std::collections::BTreeSet, BTreeSet);
    impl_opaque!(::std::collections::BinaryHeap, BinaryHeap);
    impl_opaque!(::std::marker::PhantomData, PhantomData);
    impl_opaque2!(::std::collections::HashMap, HashMap2);
    impl_opaque3!(::std::collections::HashMap, HashMap3);
}
