#![allow(unused_variables)]
#![allow(dead_code)]
use std::io::{self, Write};
use std::f64::consts::PI;           //    const PI: f64 = 3.14159265359;
use num_complex::Complex64;


pub fn sum_complex( z1 : Complex64, z2 : Complex64) -> (Complex64 , Complex64) {
    (z1 + z2, z1 - z2)
}


pub fn get_complex() -> Complex64 {

    let mut holder = String::new();
    std::io::stdin()
        .read_line(&mut holder)
        .expect("error reading input");
    let test: Vec<&str> = holder.split_whitespace().collect();
    assert_eq!(2, test.len(), "expected two numbers");
    let m: f64 = test[0].parse::<f64>().expect("'m' is not a number");
    let a: f64 = test[1].parse::<f64>().expect("'a' is not a number");
    return Complex64::from_polar(m, a * PI / 180.0);

}

// Helper function to get input from the user
pub fn get_input(prompt: &str) -> (f64, f64) {
    loop {
        println!("{}", prompt);
        io::stdout().flush().unwrap();
    
        // 2. Leer la línea completa desde el teclado
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error al leer la línea");
        
        let inputs: Vec<f64> = line.split_whitespace()
            .map(|x| x.parse::<f64>().expect("Not a number!"))
            .collect();
        // inputs is a Vec<f64> of the inputs.
        
        match inputs.as_slice() {
            [x, y] => return (*x, *y),
            _ => println!("Se recibieron muy pocos argumentos"),
        }
    }
}
