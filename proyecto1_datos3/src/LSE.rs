pub struct LinkedList {
    head: Link<T>,
}

impl LinkedList<u32> {}

impl LinkedList<String> {}

impl<T> LinkedList<T>{

    fn empty() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn push (&mut self, element: T){
        let old_head: Option<Box<Node>> = self.head.take();
        let new_head: Box<Node> = Box:: new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T>{
        self.head.take().map(|n|{
            self.head = n.next;
            n.element
        })
    }

    fn peak(&self) -> Option<&T>{

    }

}

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;