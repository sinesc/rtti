/*!
 * Run-time type information trait. Use crate rtti-derive to implement.
 *
 * **very early, probably best to stay away for now**
 *
 * Implementing `rtti()` for a custom type:
 *
 * ```
 * #[macro_use]
 * extern crate rtti_derive;
 * extern crate rtti;
 * use rtti::RTTI;
 *
 * #[derive(RTTI)]
 * struct Simple {
 *     x: u32,
 *     pub y: ::std::sync::Arc<u32>,
 *     pub(crate) z: Vec<f64>
 * }
 *
 * fn main() {
 *     println!("{:?}", Simple::rtti());
 * }
 * ```
 *
 * When implementing RTTI for a generic type, make sure generic parameters implement RTTI:
 *
 * ```
 * #[derive(RTTI)]
 * struct Generic<T> where T: RTTI {
 *     test: T,
 *     stuff: Simple,
 * }
 *
 * fn main() {
 *     println!("{:?}", Generic::<u64>::rtti());
 * }
 * ```
 */

mod types;
pub use types::*;

/// Provides run-time type information.
pub trait RTTI {
    /// Returns a Type enum describing the type.
    fn rtti() -> Type;
}

// implement built in types
mod wrapper {
    #[path="../macros.rs"]
    #[macro_use]
    mod macros;
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
    impl_opaque!(std::rc::Rc, Rc);
    impl_opaque!(std::cell::Cell, Cell);
    impl_opaque!(std::cell::RefCell, RefCell);
    impl_opaque!(std::cell::UnsafeCell, UnsafeCell);
    impl_opaque!(std::boxed::Box, Box);
    impl_opaque!(std::sync::Arc, Arc);
    impl_opaque!(std::sync::Mutex, Mutex);
    impl_opaque!(std::sync::RwLock, RwLock);
    impl_opaque!(std::sync::Weak, Weak);
    impl_opaque!(std::vec::Vec, Vec);
    impl_opaque!(std::collections::VecDeque, VecDeque);
    impl_opaque!(std::collections::LinkedList, LinkedList);
    impl_opaque!(std::collections::BTreeSet, BTreeSet);
    impl_opaque!(std::collections::BinaryHeap, BinaryHeap);
    impl_opaque!(std::marker::PhantomData, PhantomData);

}
