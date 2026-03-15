#![allow(unused)]
use std::f64::consts::PI;           //    const PI: f64 = 3.14159265359;
use num_complex::Complex64;


pub fn sum_complex( m1 : f64, a1 : f64, m2 : f64, a2 : f64) -> (Complex64 , Complex64) {
    let z1 = Complex64::from_polar(m1, a1 * PI / 180.0);
    let z2 = Complex64::from_polar(m2, a2 * PI / 180.0);
    (z1 + z2, z1 - z2)    
}