macro_rules! impl_prim {
    ($t:ty, $i:ident) => (
        impl $crate::RTTI for $t {
            #[inline(always)]
            fn rtti() -> $crate::Type {
                $crate::Type::$i($crate::Primitive {
                    name: stringify!($i).to_string(),
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
                fn rtti() -> $crate::Type {
                    $crate::Type::Opaque($crate::Opaque {
                        name: stringify!($i).to_string(),
                        tys: vec![ Box::new(T::rtti()) ],
                        size: ::std::mem::size_of::<$t<T>>(),
                    })
                }
            }
        }
    )
}