pub struct Stack<T> {
    val: Vec<T>,
    max: usize,
}

impl<T> Stack<T> {
    pub fn init() -> Self {
        Self {
            val: vec![],
            max: 1000,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.val.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.max == self.val.len()
    }

    pub fn push(&mut self, element: T) {
        if self.is_full() {
            panic!("cannot be added");
        }
        self.val.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.val.pop()
        }
    }
}

pub struct Queue<T> {
    val: Vec<T>,
    max: usize,
}

impl<T> Queue<T> {
    pub fn init() -> Self {
        Self {
            val: vec![],
            max: 1000,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.val.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.max == self.val.len()
    }

    pub fn enqueue(&mut self, element: T) {
        if self.is_full() {
            panic!("cannot be added");
        }
        self.val.push(element);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.val.remove(0))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Queue, Stack};

    #[test]
    fn test_stack() {
        let mut stack: Stack<u32> = Stack::init();
        assert_eq!(stack.val, vec![]);
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.is_full(), false);
        //
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        //
        for i in 1..=1000 {
            stack.push(i);
        }
        assert_eq!(stack.is_full(), true)
    }

    #[test]
    fn test_queue() {
        let mut queue: Queue<u32> = Queue::init();
        assert_eq!(queue.val, vec![]);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_full(), false);
        //
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        //
        for i in 1..=1000 {
            queue.enqueue(i);
        }
        assert_eq!(queue.is_full(), true)
    }
}
