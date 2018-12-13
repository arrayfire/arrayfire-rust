pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

macro_rules! zero_impl {
    ( $t:ident, $z:expr ) => {
        impl Zero for $t {
            fn zero() -> Self {
                $z
            }
        }
    };
}

zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(usize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(isize, 0);
zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

macro_rules! one_impl {
    ( $t:ident, $o:expr ) => {
        impl One for $t {
            fn one() -> Self {
                $o
            }
        }
    };
}

one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
one_impl!(usize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
one_impl!(isize, 1);
one_impl!(f32, 1.0);
one_impl!(f64, 1.0);
