/*
//Mutables e inmutables
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; //Esto daria error si la variable es inmutable
    println!("The value of x is: {}", x);
}

//Shadowing
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

//Tipos de punto flotante
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

//Operaciones numericas
fn main() {
    // suma
    let sum = 5 + 10;
    // resta
    let difference = 95.5 - 4.3;
    // multiplicación
    let product = 4 * 30;
    // división
    let quotient = 56.7 / 32.2;
    // resto o módulo
    let remainder = 43 % 5;
}

//Tipo booleano
fn main() {
    let t = true;
    let f: bool = false;
}
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
//El tipo tupla
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
//
*/
fn main() {
    //let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{} {}", first,second)
}
