use std::Boxed::Box;

#[allow(dead_code)]
struct Node<T>{
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

struct Deque<T> {
    begin:Option<Box<Node<T>>>,
    end: Option<Box<Node<T>>>,
}

impl<T> Deque<T> {
    fn new() -> Deque<T> {
        todo!()
    }
    fn push_front(&mut self, value: T){
        todo!()
    }
}

fn main(){
    println!("Hello, World!");
}
