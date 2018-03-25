macro_rules! impl_prim {
    ($t:ty, $i:ident) => (
        impl $crate::RTTI for $t {
            #[inline(always)]
            fn ctti() -> $crate::Type {
                $crate::Type::$i($crate::Primitive {
                    name: stringify!($i),
                    size: ::std::mem::size_of::<$t>(),
                })
            }
        }
    )
}

macro_rules! impl_opaque {
    ($i:path, $t:tt) => (
        #[allow(non_upper_case_globals, non_snake_case)]
        mod $t {
            use $i;
            impl<T> $crate::RTTI for $t <T> where T: $crate::RTTI {
                #[inline(always)]
                fn ctti() -> $crate::Type {
                    $crate::Type::Opaque($crate::Opaque {
                        name: stringify!($i),
                        tys: vec![ T::ctti() ],
                        size: ::std::mem::size_of::<$t<T>>(),
                    })
                }
            }
        }
    )
}

macro_rules! impl_opaque2 {
    ($i:path, $t:tt) => (
        #[allow(non_upper_case_globals, non_snake_case)]
        mod $t {
            use $i as $t;
            impl<T, U> $crate::RTTI for $t <T, U> where T: $crate::RTTI, U: $crate::RTTI {
                #[inline(always)]
                fn ctti() -> $crate::Type {
                    $crate::Type::Opaque($crate::Opaque {
                        name: stringify!($i),
                        tys: vec![ T::ctti(), U::ctti() ],
                        size: ::std::mem::size_of::<$t<T, U>>(),
                    })
                }
            }
        }
    )
}

macro_rules! impl_opaque3 {
    ($i:path, $t:tt) => (
        #[allow(non_upper_case_globals, non_snake_case)]
        mod $t {
            use $i as $t;
            impl<T, U, V> $crate::RTTI for $t <T, U, V> where T: $crate::RTTI, U: $crate::RTTI, V: $crate::RTTI {
                #[inline(always)]
                fn ctti() -> $crate::Type {
                    $crate::Type::Opaque($crate::Opaque {
                        name: stringify!($i),
                        tys: vec![ T::ctti(), U::ctti(), V::ctti() ],
                        size: ::std::mem::size_of::<$t<T, U, V>>(),
                    })
                }
            }
        }
    )
}
