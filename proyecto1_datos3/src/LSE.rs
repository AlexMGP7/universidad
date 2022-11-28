pub struct LinkedList {
    head: Link,
}

impl LinkedList{

    fn empty() -> LinkedList {
        LinkedList { head: None }
    }

    fn push (&mut self, element: u32){
        let old_head: Option<Box<Node>> = self.head.take();
        let new_head: Box<Node> = Box:: new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<u32>{
        self.head.take().map(|n|{
            self.head = n.next;
            n.element
        })
    }

    fn peak(&mut self) -> &u32{

    }

}

struct Node {
    element: u32,
    next: Link,
}