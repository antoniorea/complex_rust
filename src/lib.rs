#![allow(unused)]
use std::f64::consts::PI;           //    const PI: f64 = 3.14159265359;
use num_complex::Complex64;


pub fn sum_complex( z1 : Complex64, z2 : Complex64) -> (Complex64 , Complex64) {
    (z1 + z2, z1 - z2)
}


pub fn get_complex() -> Complex64 {
    eprint!("Please enter the values for mudule and argument in degrees, separated with a space: \n>");
    let mut holder = String::new();
    std::io::stdin()
        .read_line(&mut holder)
        .expect("error reading input");
    let test: Vec<&str> = holder.split_whitespace().collect();
    assert_eq!(2, test.len(), "expected two numbers");
    let m: f64 = test[0].parse().expect("'m' is not a number");
    let a: f64 = test[1].parse().expect("'a' is not a number");
    return Complex64::from_polar(m, a * PI / 180.0);

}
