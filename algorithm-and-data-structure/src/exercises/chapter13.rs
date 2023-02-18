use core::panic;
use std::collections::VecDeque;

pub struct Graph {
    v: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn search(&self, s: usize) -> Vec<bool> {
        let mut seen = vec![false; self.v.len()];
        self._search(s, &mut seen);
        seen
    }

    pub fn _search(&self, s: usize, seen: &mut Vec<bool>) {
        let mut todo: VecDeque<usize> = VecDeque::new();

        seen[s] = true;
        todo.push_back(s);

        while !todo.is_empty() {
            let v = todo.pop_front().unwrap();
            for x in &self.v[v] {
                match seen.get(*x) {
                    Some(_) => continue,
                    None => {
                        seen[*x] = true;
                        todo.push_back(*x);
                    }
                }
            }
        }
    }

    pub fn search_by_recursive(&self, s: usize) -> Vec<bool> {
        let n = self.v.len();
        let mut seen = vec![false; n];
        self._search_by_resursive(s, &mut seen);
        seen
    }

    fn _search_by_resursive(&self, v: usize, seen: &mut Vec<bool>) {
        seen[v] = true;
        for x in &self.v[v] {
            match seen.get(*x) {
                Some(_) => continue,
                None => self._search_by_resursive(*x, seen),
            }
        }
    }

    pub fn dfs(&self, s: usize) -> Vec<i32> {
        let n = self.v.len();
        let mut dist: Vec<i32> = vec![-1; n];
        let mut queue: VecDeque<usize> = VecDeque::new();

        dist[s] = 0;
        queue.push_front(s);

        while queue.is_empty() {
            let v = queue.pop_front().unwrap();
            for x in &self.v[v] {
                if dist[*x] != -1 {
                    continue;
                }

                dist[*x] = dist[v] + 1;
                queue.push_back(*x);
            }
        }

        dist
    }
}

pub struct BipartiteGraph {
    v: Vec<Vec<usize>>,
}

impl BipartiteGraph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    fn _dfs(&self, color: &mut Vec<isize>, v: usize, cur: usize) -> bool {
        for next_v in &self.v[v] {
            let val = color[*next_v];
            match val {
                -1 => self.dfs(*next_v, 1 - cur),
                0 | 1 => {
                    if val == cur as isize {
                        return false;
                    }
                    continue;
                }
                _ => panic!(),
            };
        }
        return true;
    }

    pub fn dfs(&self, v: usize, cur: usize) -> bool {
        let mut color: Vec<isize> = vec![-1; self.v.len()];
        self._dfs(&mut color, v, cur)
    }
}

pub struct FindPathGraph {
    v: Vec<Vec<usize>>,
}

impl FindPathGraph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    fn _dfs(&self, v: usize, seen: &mut Vec<bool>) {
        seen[v] = true;
        for next_v in &self.v[v] {
            if seen[*next_v] {
                continue;
            }
            self._dfs(*next_v, seen);
        }
    }

    pub fn dfs(&self, s: usize, t: usize) -> bool {
        let mut seen = vec![false; self.v.len()];
        self._dfs(s, &mut seen);
        return seen[t];
    }
}

struct TopologicalSortGraph {
    v: Vec<Vec<usize>>,
}

impl TopologicalSortGraph {
    pub fn new(v: Vec<Vec<usize>>) -> Self {
        Self { v: v }
    }
}

#[cfg(test)]
mod test {
    use super::{BipartiteGraph, FindPathGraph, Graph};

    #[test]
    fn test_search() {
        let mut graph = Graph::new();
        graph.v.push(vec![1, 2]);
        graph.v.push(vec![3]);
        graph.v.push(vec![3]);
        let actual = graph.search_by_recursive(0);
        let expected = vec![true, false, false];
        assert_eq!(actual, expected);
        let actual = graph.search(0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        graph.v.push(vec![1, 2]);
        graph.v.push(vec![3]);
        graph.v.push(vec![3]);
        let actual = graph.dfs(0);
        let expected = vec![0, -1, -1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bipartite_graph() {
        let mut graph = BipartiteGraph::new();
        graph.v.push(vec![1, 2]);
        graph.v.push(vec![3]);
        graph.v.push(vec![3]);
        graph.v.push(vec![]);
        let actual = graph.dfs(0, 1);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_path_graph() {
        let mut graph = FindPathGraph::new();
        graph.v.push(vec![1, 2]);
        graph.v.push(vec![3]);
        graph.v.push(vec![4]);
        graph.v.push(vec![5]);
        graph.v.push(vec![5]);
        graph.v.push(vec![]);

        let actual = graph.dfs(1, 2);
        let expected = false;
        assert_eq!(actual, expected);

        let actual = graph.dfs(1, 3);
        let expected = true;
        assert_eq!(actual, expected);
    }
}
