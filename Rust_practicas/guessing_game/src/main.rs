#![allow(non_snake_case)]

use std::io;
fn main() {
    println!("¡Adivina el número!");
    println!("Por favor, introduce tu adivinanza.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Fallo al leer la línea.");
    println!("Tu adivinanza: {}", guess);
}