pub struct Edge {
    from: isize,
    to: isize,
}

pub struct UnionFind {
    par: Vec<isize>,
    siz: Vec<isize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            siz: vec![-1; n],
        }
    }

    fn root(&mut self, x: isize) -> isize {
        match self.par[x as usize] {
            -1 => x,
            _ => {
                self.par[x as usize] = self.root(self.par[x as usize]);
                self.par[x as usize]
            }
        }
    }

    fn is_same(&mut self, x: isize, y: isize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: isize, y: isize) -> bool {
        let mut parent = self.root(x);
        let mut child = self.root(y);

        if parent == child {
            return false;
        }
        if self.siz[parent as usize] < self.siz[child as usize] {
            std::mem::swap(&mut parent, &mut child);
        }
        self.par[child as usize] = parent;
        self.siz[parent as usize] += self.siz[child as usize];
        true
    }

    fn size(&mut self, x: isize) -> isize {
        let root = self.root(x);
        self.siz[root as usize]
    }
}

pub fn find_bridges(edge: Vec<Edge>, nodes: isize) -> usize {
    let mut result = 0;

    for e in 0..edge.len() {
        let mut ufind: UnionFind = UnionFind::new(nodes as usize);
        for i in 0..edge.len() {
            if i == e {
                continue;
            }
            ufind.unite(edge[i].from, edge[i].to);
        }

        let mut count = 0;
        for v in 0..nodes {
            if ufind.root(v) == v {
                count += 1;
            }
        }
        if count > 1 {
            result += 1;
        }
    }
    result
}

pub fn decayed_bridges(edges: Vec<Edge>, nodes: isize) -> Vec<isize> {
    let mut ans = nodes * (nodes - 1) / 2;
    let mut result = vec![ans; edges.len()];
    let mut ufind = UnionFind::new(nodes as usize);
    for m in (0..edges.len()).rev() {
        result[m] = ans;
        let edge = &edges[m];
        if ufind.is_same(edge.from, edge.to) {
            continue;
        }
        println!("m: {}", m);
        println!(
            "ans, ufind.size(edge.from), ufind.size(edge.to): {}, {}, {}",
            ans,
            ufind.size(edge.from),
            ufind.size(edge.to)
        );
        ans -= ufind.size(edge.from) * ufind.size(edge.to);
        ufind.unite(edge.from, edge.to);
        result[m] = ans;
    }

    result
}

#[cfg(test)]
mod test {
    use super::{decayed_bridges, find_bridges, Edge};

    #[test]
    fn test_find_num_of_bridges() {
        let bridges = vec![
            Edge { from: 0, to: 1 },
            Edge { from: 1, to: 2 },
            Edge { from: 2, to: 3 },
            Edge { from: 3, to: 4 },
            Edge { from: 4, to: 5 },
        ];
        assert_eq!(find_bridges(bridges, 6), 5);
    }

    #[test]
    #[ignore]
    fn test_decayed_bridges() {
        let bridges = vec![
            Edge { from: 2, to: 3 },
            Edge { from: 1, to: 2 },
            Edge { from: 5, to: 6 },
            Edge { from: 3, to: 4 },
            Edge { from: 4, to: 5 },
        ];
        assert_eq!(decayed_bridges(bridges, 6), vec![8, 9, 12, 14, 15]);
    }
}
