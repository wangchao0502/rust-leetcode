#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs
pub struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn with_capacity(n: usize) -> Self {
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = i;
        }
        UnionFind {
            parents: v,
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        while self.parents[p] != p {
            self.parents[p] = self.parents[self.parents[p]];
            p = self.parents[p];
        }
        p
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i != j {
            self.count -= 1;
            if self.size[i] > self.size[j] {
                self.parents[j] = i;
                self.size[i] += self.size[j];
            } else {
                self.parents[i] = j;
                self.size[j] += self.size[i];
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

// answers
impl Solution {
    pub fn p5128_are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // codes
        let mut uf = UnionFind::with_capacity(n as usize + 1);
        let mut ans = vec![];

        for i in (threshold + 1)..(n + 1) {
            // 筛法，倍数
            for j in ((i * 2)..(n + 1)).step_by(i as usize) {
                uf.union(i as usize, j as usize);
            }
        }

        for query in queries.iter() {
            ans.push(uf.connected(query[0] as usize, query[1] as usize));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5128_are_connected_t1() {
        assert_eq!(
            Solution::p5128_are_connected(6, 2, vec![vec![1, 4], vec![2, 5], vec![3, 6]]),
            vec![false, false, true]
        );
    }

    #[test]
    fn p5128_are_connected_t2() {
        assert_eq!(
            Solution::p5128_are_connected(
                5,
                1,
                vec![vec![4, 5], vec![3, 2], vec![2, 3], vec![3, 4]]
            ),
            vec![false, false, false, false]
        );
    }
}
