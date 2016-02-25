pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }

impl One for u8 { fn one() -> Self { 1 } }
impl One for u16 { fn one() -> Self { 1 } }
impl One for u32 { fn one() -> Self { 1 } }
impl One for u64 { fn one() -> Self { 1 } }
impl One for usize { fn one() -> Self { 1 } }
impl One for i8 { fn one() -> Self { 1 } }
impl One for i16 { fn one() -> Self { 1 } }
impl One for i32 { fn one() -> Self { 1 } }
impl One for i64 { fn one() -> Self { 1 } }
impl One for isize { fn one() -> Self { 1 } }
impl One for f32 { fn one() -> Self { 1.0 } }
impl One for f64 { fn one() -> Self { 1.0 } }
