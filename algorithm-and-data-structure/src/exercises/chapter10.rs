pub struct Node {
    to: usize,
    weight: u32,
}

impl Node {
    pub fn new(to: usize, weight: u32) -> Self {
        Self { to, weight }
    }
}

type Graph = Vec<Vec<Node>>;

struct Heap {
    heap: Vec<u32>,
}

impl Heap {
    pub fn new() -> Self {
        Self { heap: vec![] }
    }

    pub fn push(&mut self, element: u32) {
        self.heap.push(element);
        let mut index = self.heap.len() - 1;
        while index > 0 {
            let p = (index - 1) / 2;
            if self.heap[p] >= element {
                break;
            } else {
                self.heap[index] = self.heap[p];
                index = p;
            }
        }
    }

    pub fn top(&self) -> Option<u32> {
        if self.heap.len() == 0 {
            None
        } else {
            Some(self.heap[0])
        }
    }

    pub fn pop(&mut self) {
        if self.heap.len() == 0 {
            return;
        }
        //
        let x = self.heap.pop().unwrap();
        let mut index = 0;
        while index * 2 + 1 < self.heap.len() {
            let mut child1 = index * 2 + 1;
            let child2 = index * 2 + 2;
            if child2 < self.heap.len() && self.heap[child2] > self.heap[child1] {
                child1 = child2;
            }
            if self.heap[child1] <= x {
                break;
            } else {
                self.heap[index] = self.heap[child1];
                index = child1;
            }
        }
        self.heap[index] = x;
    }
}

#[cfg(test)]
mod test {
    use super::{Graph, Heap, Node};

    #[test]
    fn test_graph() {
        let mut graph: Graph = vec![vec![]];
        let node0 = vec![Node::new(1, 5), Node::new(2, 3)];
        graph.push(node0);
        let node1 = vec![Node::new(3, 1)];
        graph.push(node1);
        let node2 = vec![Node::new(3, 5)];
        graph.push(node2);
    }

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        heap.push(10);
        heap.push(4);
        heap.push(5);
        heap.push(1);
        //
        assert_eq!(heap.top(), Some(10));
        heap.pop();
        assert_eq!(heap.top(), Some(5));
        heap.pop();
        assert_eq!(heap.top(), Some(4));
    }
}
