#![allow(non_snake_case)]

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("¡Adivina el número!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("El numero secreto es {}", secret_number);
    loop {
        println!("Por favor, introduce tu adivinanza.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Fallo al leer la línea.");
        let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
        println!("Tu adivinanza: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}