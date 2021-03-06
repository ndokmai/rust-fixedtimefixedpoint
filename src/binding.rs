#![allow(non_camel_case_types)]

pub type fixed = u64;
extern "C" {
    pub fn fix_is_neg(op1: fixed) -> i8;
}
extern "C" {
    pub fn fix_is_nan(op1: fixed) -> i8;
}
extern "C" {
    pub fn fix_is_inf_pos(op1: fixed) -> i8;
}
extern "C" {
    pub fn fix_is_inf_neg(op1: fixed) -> i8;
}
extern "C" {
    pub fn fix_eq(op1: fixed, op2: fixed) -> i8;
}
extern "C" {
    pub fn fix_eq_nan(op1: fixed, op2: fixed) -> i8;
}
extern "C" {
    pub fn fix_ne(op1: fixed, op2: fixed) -> i8;
}
extern "C" {
    pub fn fix_cmp(op1: fixed, op2: fixed) -> i8;
}
extern "C" {
    pub fn fix_le(op1: fixed, op2: fixed) -> u8;
}
extern "C" {
    pub fn fix_ge(op1: fixed, op2: fixed) -> u8;
}
extern "C" {
    pub fn fix_lt(op1: fixed, op2: fixed) -> u8;
}
extern "C" {
    pub fn fix_gt(op1: fixed, op2: fixed) -> u8;
}
extern "C" {
    pub fn fix_neg(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_abs(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_add(op1: fixed, op2: fixed) -> fixed;
}
extern "C" {
    pub fn fix_sub(op1: fixed, op2: fixed) -> fixed;
}
extern "C" {
    pub fn fix_mul(op1: fixed, op2: fixed) -> fixed;
}
extern "C" {
    pub fn fix_div(op1: fixed, op2: fixed) -> fixed;
}
extern "C" {
    pub fn fix_floor(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_ceil(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_exp(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_ln(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_log2(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_log10(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_sqrt(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_pow(x: fixed, y: fixed) -> fixed;
}
extern "C" {
    pub fn fix_sin(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_cos(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_tan(op1: fixed) -> fixed;
}
extern "C" {
    pub fn fix_convert_from_double(d: f64) -> fixed;
}
extern "C" {
    pub fn fix_convert_to_double(op1: fixed) -> f64;
}
extern "C" {
    pub fn fix_convert_from_int64(i: i64) -> fixed;
}
extern "C" {
    pub fn fix_convert_to_int64(op1: fixed) -> i64;
}
extern "C" {
    pub fn fix_round_up_int64(op1: fixed) -> i64;
}
extern "C" {
    pub fn fix_ceil64(op1: fixed) -> i64;
}
extern "C" {
    pub fn fix_floor64(op1: fixed) -> i64;
}
