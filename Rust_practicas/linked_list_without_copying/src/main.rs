use std::Boxed::Box;

#[allow(dead_code)]
struct Node<T>{
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}
