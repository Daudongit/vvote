pub(crate) trait NumberType{
    fn type_of(&self) -> String;
}

macro_rules! number_type {
    ($($ty:ty),*) => {
        $(
            impl NumberType for $ty {
                fn type_of(&self) -> String {
                    stringify!($ty).into()
                }
            }
        )*
    }
}

#[rustfmt::skip]
number_type!(u8, i8, u16, i16, u32, i32, i64, u64, usize, isize, i128, u128);