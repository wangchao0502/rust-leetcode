#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// An undirected, connected tree with N nodes labelled 0...N-1 and N-1 edges are given.
// The ith edge connects nodes edges[i][0] and edges[i][1] together.
// Return a list ans, where ans[i] is the sum of the distances between node i and all other nodes.

// Input: N = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
// Output: [8,12,6,10,10,10]
// Explanation:
// Here is a diagram of the given tree:
//   0
//  / \
// 1   2
//    /|\
//   3 4 5
// We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
// equals 1 + 1 + 2 + 2 + 2 = 8.  Hence, answer[0] = 8, and so on.

// answers
// hard, double dfs
impl Solution {
    pub fn p834_sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // codes
        fn dfs(
            tree: &[Vec<usize>],
            count: &mut Vec<i32>,
            ans: &mut Vec<i32>,
            root: usize,
            pre: Option<usize>,
        ) {
            for &i in &tree[root] {
                if pre.is_some() && i == pre.unwrap() {
                    continue;
                }
                dfs(tree, count, ans, i, Some(root));
                count[root] += count[i];
                ans[root] += ans[i] + count[i];
            }
            count[root] += 1;
        }

        fn dfs2(
            tree: &[Vec<usize>],
            count: &mut Vec<i32>,
            ans: &mut Vec<i32>,
            root: usize,
            pre: Option<usize>,
        ) {
            for &i in &tree[root] {
                if pre.is_some() && i == pre.unwrap() {
                    continue;
                }
                ans[i] = ans[root] - count[i] + count.len() as i32 - count[i];
                dfs2(tree, count, ans, i, Some(root));
            }
        }

        let mut tree = vec![vec![]; n as usize];
        let mut ans = vec![0; n as usize];
        let mut count = vec![0; n as usize];

        for e in &edges {
            tree[e[0] as usize].push(e[1] as usize);
            tree[e[1] as usize].push(e[0] as usize);
        }

        dfs(&tree, &mut count, &mut ans, 0, None);
        dfs2(&tree, &mut count, &mut ans, 0, None);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p834_sum_of_distances_in_tree_t1() {
        assert_eq!(
            Solution::p834_sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
            ),
            vec![8, 12, 6, 10, 10, 10]
        );
    }
}
