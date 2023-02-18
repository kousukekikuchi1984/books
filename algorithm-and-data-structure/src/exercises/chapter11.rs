pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x; // root
        }
        self.par[x] = self.root(self.par[x]); // path-compression
        self.par[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }
        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }
        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.par[root]
    }
}

#[cfg(test)]
mod test {
    use super::UnionFind;

    #[test]
    fn test_union_find() {
        let mut union_find = UnionFind::new(4);
        let m = 2;
        let par = vec![1, 1];
        let siz = vec![2, 3];
        for i in 0..m {
            union_find.unite(par[i], siz[i]);
        }
        let mut counter = 0;
        for i in 0..m {
            if union_find.unite(0, i) {
                counter += 1;
            }
        }
        assert_eq!(counter, 1);
    }
}
