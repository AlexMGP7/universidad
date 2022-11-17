/* //Mutables e inmutables
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; //Esto daria error si la variable es inmutable
    println!("The value of x is: {}", x);
}
*/
//Shadowing
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
