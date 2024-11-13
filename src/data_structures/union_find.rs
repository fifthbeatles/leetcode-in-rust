use std::cmp::Ordering;

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    /**
     * 查找根节点，并进行路径压缩
     */
    pub fn find(&mut self, p: usize) -> usize {
        if self.parent[p] != p {
            self.parent[p] = self.find(self.parent[p]);
        }
        self.parent[p]
    }

    /**
     * 合并
     */
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        if root_p != root_q {
            match self.rank[root_p].cmp(&self.rank[root_q]) {
                Ordering::Greater => self.parent[root_q] = root_p,
                Ordering::Less => self.parent[root_p] = root_q,
                Ordering::Equal => {
                    self.parent[root_q] = root_p;
                    self.rank[root_p] += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let mut uf = UnionFind::new(10);
        uf.union(1, 2);
        uf.union(3, 4);
        uf.union(2, 4);
        assert_eq!(uf.find(1), uf.find(3));
    }
}
