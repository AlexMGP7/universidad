#![allow(non_snake_case)]

extern crate rand;

use std::io;
use rand::Rng;
fn main() {
    println!("¡Adivina el número!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);
    println!("Por favor, introduce tu adivinanza.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Fallo al leer la línea.");
    println!("Tu adivinanza: {}", guess);
}