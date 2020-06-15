mod binding;

include!(concat!(env!("OUT_DIR"), "/bits.rs"));
use binding::*;
use std::cmp::Ordering;
use std::ops::*;

use FIX_FLAG_BITS as FLAG_BITS;
pub const EPSILON: Fixed = Fixed(1u64 << FLAG_BITS);
pub const EPSILON_NEG: Fixed = Fixed(!((1u64 << FLAG_BITS) - 1));
pub const ZERO: Fixed = Fixed(0);
pub const MAX: Fixed = Fixed((((1 as fixed) << (FIX_BITS - 1)) - 1) & FIX_DATA_BIT_MASK);
pub const MIN: Fixed = Fixed(((1 as fixed) << (FIX_BITS - 1)) & FIX_DATA_BIT_MASK);

const FIX_BITS: usize = 8 * std::mem::size_of::<fixed>();
const FIX_DATA_BIT_MASK: fixed = 0xFFFFFFFFFFFFFFFC;

#[derive(Clone, Copy, Debug)]
pub struct Fixed(fixed);

impl Fixed {
    /// Returns true if the numbers are equal (and also if they are both NaN)
    pub fn eq_nan(self, other: Self) -> bool {
        unsafe { fix_eq_nan(self.0, other.0) != 0 }
    }

    pub fn floor(self) -> Self {
        Self(unsafe { fix_floor(self.0) })
    }

    pub fn ceil(self) -> Self {
        Self(unsafe { fix_ceil(self.0) })
    }

    pub fn floor_int64(self) -> i64 {
        unsafe { fix_floor64(self.0) }
    }

    pub fn ceil_int64(self) -> i64 {
        unsafe { fix_ceil64(self.0) }
    }

    pub fn round_int64(self) -> i64 {
        unsafe { fix_round_up_int64(self.0) }
    }

    pub fn abs(self) -> Self {
        Self(unsafe { fix_abs(self.0) })
    }

    /// Computes x^y. Note that this is undefined when self < 0 and other is not an integer, and will return NaN.
    pub fn pow(self, other: Self) -> Self {
        Self(unsafe { fix_pow(self.0, other.0) })
    }

    pub fn sqrt(self) -> Self {
        Self(unsafe { fix_sqrt(self.0) })
    }

    pub fn exp(self) -> Self {
        Self(unsafe { fix_exp(self.0) })
    }

    pub fn ln(self) -> Self {
        Self(unsafe { fix_ln(self.0) })
    }

    pub fn log2(self) -> Self {
        Self(unsafe { fix_log2(self.0) })
    }

    pub fn log10(self) -> Self {
        Self(unsafe { fix_log10(self.0) })
    }

    /// Accurate to 2^-57.
    pub fn sin(self) -> Self {
        Self(unsafe { fix_sin(self.0) })
    }

    /// Accurate to 2^-57.
    pub fn cos(self) -> Self {
        Self(unsafe { fix_cos(self.0) })
    }

    /// Accurate to 2^-57.
    pub fn tan(self) -> Self {
        Self(unsafe { fix_tan(self.0) })
    }

    pub fn is_nan(self) -> bool {
        unsafe { fix_is_nan(self.0) != 0 }
    }

    pub fn is_pos_infinite(self) -> bool {
        unsafe { fix_is_inf_pos(self.0) != 0 }
    }

    pub fn is_neg_infinite(self) -> bool {
        unsafe { fix_is_inf_neg(self.0) != 0 }
    }

    pub fn is_sign_negative(self) -> bool {
        unsafe { fix_is_neg(self.0) != 0 }
    }
}

impl From<f64> for Fixed {
    fn from(d: f64) -> Self {
        Self(unsafe { fix_convert_from_double(d) })
    }
}

impl From<i64> for Fixed {
    fn from(d: i64) -> Self {
        Self(unsafe { fix_convert_from_int64(d) })
    }
}

impl Into<f64> for Fixed {
    fn into(self) -> f64 {
        unsafe { fix_convert_to_double(self.0) }
    }
}

impl Into<i64> for Fixed {
    fn into(self) -> i64 {
        unsafe { fix_convert_to_int64(self.0) }
    }
}

impl Default for Fixed {
    fn default() -> Self {
        ZERO
    }
}

impl PartialEq for Fixed {
    fn eq(&self, other: &Self) -> bool {
        unsafe { fix_eq(self.0, other.0) != 0 }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe { fix_ne(self.0, other.0) != 0 }
    }
}

impl PartialOrd for Fixed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match unsafe { fix_cmp(self.0, other.0) } {
            -1 => Some(Ordering::Less),
            0 => Some(Ordering::Equal),
            1 => {
                if self.is_nan() || other.is_nan() {
                    None
                } else {
                    Some(Ordering::Greater)
                }
            }
            _ => panic!("This should never happen"),
        }
    }

    fn le(&self, other: &Self) -> bool {
        unsafe { fix_le(self.0, other.0) != 0 }
    }

    fn ge(&self, other: &Self) -> bool {
        unsafe { fix_ge(self.0, other.0) != 0 }
    }

    fn lt(&self, other: &Self) -> bool {
        unsafe { fix_lt(self.0, other.0) != 0 }
    }

    fn gt(&self, other: &Self) -> bool {
        unsafe { fix_gt(self.0, other.0) != 0 }
    }
}

impl Add for Fixed {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(unsafe { fix_add(self.0, other.0) })
    }
}

impl Sub for Fixed {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(unsafe { fix_sub(self.0, other.0) })
    }
}

impl Mul for Fixed {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(unsafe { fix_mul(self.0, other.0) })
    }
}

impl Div for Fixed {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(unsafe { fix_div(self.0, other.0) })
    }
}

impl Neg for Fixed {
    type Output = Self;
    fn neg(self) -> Self {
        Self(unsafe { fix_neg(self.0) })
    }
}

impl std::fmt::Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let i: f64 = (*self).into();
        i.fmt(f)
    }
}
