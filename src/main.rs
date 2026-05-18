// Reading 4 numbers from command line
// crate num-complex
/*
    Este programa suma y resta dos numeros complejos en coord. polares
    los argumentos deben estar expresados en grados
$ ./complex_rs
Please enter the values for mudule and argument in degrees, separated with a space
First coplex number :
12.04 48.36
Second coplex number :
8.6 54.46

z1 + z2:  El mod. de z es : 20.61; el arg. de z en grados es : 50.90    - Coord Cart. : 13.00 + 16.00 j
z1 - z2:  El mod. de z es : 3.61; el arg. de z en grados es : 33.68     - Coord Cart. : 3.00 + 2.00 j
*/
#![allow(dead_code)]
#![allow(unused_variables)]
use complex_rust;            // functions complex_rust::get_complex() and complex_rust::sum-complex
use std::f64::consts::PI;           //    const PI: f64 = 3.14159265359;
use num_complex::Complex64;


fn main() {
    println!("Please enter the values for modulus and argument (in degrees), separated with a space");
    let (m1, a1) = complex_rust::get_input("First complex number : ");
    let  z1 : Complex64  = Complex64::from_polar(m1, a1 * PI / 180.0);
    let (m2, a2) = complex_rust::get_input("Second complex number : ");
    let  z2 : Complex64  = Complex64::from_polar(m2, a2 * PI / 180.0);

    //let (z, w) = complex_rust::sum_complex( z1, z2 );
    let z = z1 + z2;
    let w = z1 - z2;

    print!("\nz1 + z2:  El mod. de z es : {:.2}; el arg. de z en grados es : {:.2}", z.norm(), z.arg() * 180.0 / PI);
    println!("\t- Coord Cart. : {:.2} + {:.2} j", z.re, z.im);
    print!("z1 - z2:  El mod. de z es : {:.2}; el arg. de z en grados es : {:.2}", w.norm(), w.arg() * 180.0 / PI);
    println!("\t- Coord Cart. : {:.2} + {:.2} j", w.re, w.im);
}
