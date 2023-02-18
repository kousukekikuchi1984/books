use std::collections::LinkedList;

pub struct Queue<T> {
    val: LinkedList<T>,
}

pub struct Stack<T> {
    val: LinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            val: LinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.val.len() == 0
    }

    pub fn push(&mut self, element: T) {
        if self.is_empty() {
            let mut val: LinkedList<T> = LinkedList::new();
            val.push_back(element);
            self.val = val;
        } else {
            self.val.push_back(element);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.val.pop_back()
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            val: LinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.val.len() == 0
    }

    pub fn enqueue(&mut self, element: T) {
        if self.is_empty() {
            let mut val: LinkedList<T> = LinkedList::new();
            val.push_back(element);
            self.val = val;
        } else {
            self.val.push_back(element);
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.val.pop_front()
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Queue, Stack};
    #[test]
    fn test_stack_and_queue() {
        let mut stack: Stack<u32> = Stack::new();
        assert_eq!(stack.is_empty(), true);
        //
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        let mut queue: Queue<u32> = Queue::new();
        assert_eq!(queue.is_empty(), true);
        //
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }
}
