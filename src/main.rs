// Reading 4 numbers from command line
// crate num-complex
/*    
    Este programa suma y resta dos numeros complejos en coord. polares
    los argumentos deben estar expresados en grados
$ ./complex_rs 
Introduzca mod y arg del primer y segundo num complejo :
12.04 48.36 8.6 54.46

z1 + z2:  El mod. de z es : 20.61; el arg. de z en grados es : 50.90    - Coord Cart. : 13.00 + 16.00j
z1 - z2:  El mod. de z es : 3.61; el arg. de z en grados es : 33.68     - Coord Cart. : 3.00 + 2.00j
*/
use complex_rust::sum_complex;
use std::io;
use std::f64::consts::PI;           //    const PI: f64 = 3.14159265359;
//  use num_complex::Complex64;


fn main() {
    let mut m1 : f64  = 0.0;
    let mut m2 : f64  = 0.0; 
    let mut a1 : f64  = 0.0; 
    let mut a2 : f64  = 0.0;
    println!("Introduzca mod y arg del primer y segundo num complejo :");

    let mut entrada = String::new();

    // 1. Leer la línea completa
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la línea");

    // 2. Dividir la cadena, parsear y recolectar en un vector
    let numeros: Vec<f64> = entrada
        .split_whitespace() // Divide por espacios, tabs o saltos de línea
        .map(|s| s.parse().expect("Parse fallido, asegúrate de introducir números"))
        .collect();

    // 3. Verificar que se hayan introducido al menos dos números
    if numeros.len() >= 4 {
        m1 = numeros[0];
        a1 = numeros[1];
        m2 = numeros[2];
        a2 = numeros[3];
    } else {
        println!("Error: Necesitas introducir cuatro números.");
    }
     
    let (z, w) = sum_complex( m1, a1, m2, a2);

    print!("\nz1 + z2:  El mod. de z es : {:.2}; el arg. de z en grados es : {:.2}", z.norm(), z.arg() * 180.0 / PI);
    println!("\t- Coord Cart. : {:.2} + {:.2}j", z.re, z.im);
    print!("z1 - z2:  El mod. de z es : {:.2}; el arg. de z en grados es : {:.2}", w.norm(), w.arg() * 180.0 / PI);
    println!("\t- Coord Cart. : {:.2} + {:.2}j", w.re, w.im);
}

