use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back( 1); // or push_front
    list.push_back( 2);
    list.push_back( 3);
    println!("Before peek: {:?}", list);
    println!("peeked {:?}", list.back()); // or front
    println!("After peek but before pop: {:?}", list);
    list.pop_back(); // or push_front
    println!("After pop: {:?}", list);
}