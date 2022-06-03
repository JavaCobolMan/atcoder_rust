use proconio::marker::Usize1;
use proconio::*;

enum Node {
    Root(usize),
    Cnt(usize),
}
impl Copy for Node {}
impl Clone for Node {
    fn clone(&self) -> Self {
        *self
    }
}

struct UnionFind(Vec<Node>);
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind(vec![Node::Cnt(1); n])
    }

    fn get_root_index(&mut self, n: usize) -> usize {
        let tmp: Node = self.0[n];
        match tmp {
            Node::Root(x) => {
                let tmp1 = self.get_root_index(x);
                self.0[n] = Node::Root(tmp1);
                tmp1
            },
            Node::Cnt(_) => n,
        }
    }

    fn get_root_cnt(&mut self, n: usize) -> usize {
        let tmp = self.get_root_index(n);
        let tmp: Node = self.0[tmp];
        match tmp {
            Node::Root(x) => self.get_root_cnt(x),
            Node::Cnt(x) => x,
        }
    }

    fn is_root(&mut self, n: usize) -> bool {
        if n == self.get_root_index(n) {
            return true;
        } else {
            return false;
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let cnt = self.get_root_cnt(x) + self.get_root_cnt(y);
        if self.is_root(x) && self.is_root(y) {
            self.0[x] = Node::Cnt(cnt);
            self.0[y] = Node::Root(x);
        } else if self.is_root(x) && !self.is_root(y) {
            self.0[x] = Node::Cnt(cnt);
            let tmp = self.get_root_index(y);
            self.0[tmp] = Node::Root(x);
        } else if !self.is_root(x) && self.is_root(y) {
            let tmp = self.get_root_index(x);
            self.0[tmp] = Node::Root(y);
            self.0[y] = Node::Cnt(cnt);
        } else {
            let tmp1 = self.get_root_index(x);
            let tmp2 = self.get_root_index(y);
            self.0[tmp1] = Node::Cnt(cnt);
            self.0[tmp2] = Node::Root(tmp1);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, usize); n-1],
    }

    let mut uf = UnionFind::new(n);

    uvw.sort_unstable_by_key(|t| t.2);
    let mut res = 0;
    for (u, v, w) in uvw {
        res += w * uf.get_root_cnt(u) * uf.get_root_cnt(v);
        uf.unite(u, v);
    }

    println!("{}", res);
}
