/*
fn main() {
    let number = 3;
    if number < 5 {
    println!("condition was true");
    } else {
    println!("condition was false");
    }
}

fn main() {
    let number = 6;
    if number % 4 == 0 {
    println!("number is divisible by 4");
    } else if number % 3 == 0 {
    println!("number is divisible by 3");
    } else if number % 2 == 0 {
    println!("number is divisible by 2");
    } else {
    println!("number is not divisible by 4, 3, or 2");
    }
}

fn main() {
    let mut counter = 0;
    let result = loop {
    counter += 1;
    if counter == 10 {
    break counter * 2;
    }
    };
    assert_eq!(result, 20);
    println!("{}",result)
}
*/
fn main() {
    let mut number = 3;
    while number != 0 {
    println!("{}!", number);
    number = number - 1;
    }
    println!("LIFTOFF!!!");
}