pub fn vector() {
    let mut vector = vec![4, 3, 12, 7, 11, 1, 9, 8, 14, 6];
    println!("{}", vector[0]);
    println!("{}", vector[2]);
    vector[2] = 5;
    println!("{}", vector[2]);
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap();
            self.head = head_node.next;
            Some(head_node.data)
        } else {
            None
        }
    }
}

// DoublyNode is implemented as LinkedList on built-in library

#[cfg(test)]
mod test {
    use super::{vector, SimpleLinkedList};

    #[test]
    fn test_vector() {
        vector();
    }

    #[test]
    fn test_simple_linked_list() {
        let mut actual: SimpleLinkedList<u32> = SimpleLinkedList::new();
        actual.push(1);
        actual.push(2);
        actual.push(3);

        assert_eq!(actual.pop(), Some(3));
        assert_eq!(actual.pop(), Some(2));
        assert_eq!(actual.pop(), Some(1));
        assert_eq!(actual.pop(), None);
    }
}
