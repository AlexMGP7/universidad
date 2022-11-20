use std::{borrow::BorrowMut, mem::take};

struct QueueNode<T> {
    value: T,
    next: Option<Box<QueueNode<T>>>,
}

impl<T> QueueNode<T> {
    fn new(value: T) -> QueueNode<T> {
        QueueNode { value, next: None }
    }
}

pub struct Queue<T> {
    end: Option<QueueNode<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { end: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.end {
            None => true,
            _ => false,
        }
    }

    pub fn add(&mut self, value: T) {
        let new_node = QueueNode::new(value);
        if let Some(end) = &mut self.end {
            let mut start = end;
            loop {
                if let Some(_) = &start.next {
                    start = (start.next.as_mut().unwrap()).borrow_mut();
                } else {
                    break;
                }
            }
            start.next = Some(Box::new(new_node));
        } else {
            self.end = Some(new_node);
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if !self.is_empty() {
            let end = take(&mut self.end).unwrap();
            if let Some(next) = end.next {
                self.end = Some(*next);
            }
            Some(end.value)
        } else {
            None
        }
    }
}

fn main (){
    let mut q:Queue<u32> = Queue::<u32>::new();
    q.add(1);
    q.add(2);
    q.add(3);
    loop{
        let m=q.remove();
        match m {
            Some(valor) => println!("{}",valor),
            None=> break
        };
        continue;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::<u32>::new();
        q.add(1);
        q.add(2);
        q.add(3);
        assert_eq!(q.remove(), Some(1));
        assert_eq!(q.remove(), Some(2));
        assert_eq!(q.remove(), Some(3));
        assert!(q.is_empty());
    }
}