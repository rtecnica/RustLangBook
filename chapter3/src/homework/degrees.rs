/*
 *
 *
 *
 *
 *
 *
 *
 *
 *
 */


pub fn c_to_f(degrees_c: f32) -> f32 {
    degrees_c * 1.8 + 32.0
}

pub fn f_to_c(degrees_c: f32) -> f32 {
    (degrees_c - 32.0) / 1.8
}
