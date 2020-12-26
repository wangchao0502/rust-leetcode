#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是，每个孩子最多只能给一块饼干。
// 对每个孩子 i，都有一个胃口值 g[i]，这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j，
// 都有一个尺寸 s[j] 。如果 s[j] >= g[i]，我们可以将这个饼干 j 分配给孩子 i ，
// 这个孩子会得到满足。你的目标是尽可能满足越多数量的孩子，并输出这个最大数值。

// answers
impl Solution {
    pub fn p455_find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        // codes
        g.sort();
        s.sort();

        let mut ans = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                ans += 1;
                j += 1;
                i += 1;
            } else {
                while j < s.len() && s[j] < g[i] {
                    j += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p455_find_content_children_t1() {
        assert_eq!(
            Solution::p455_find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }

    #[test]
    fn p455_find_content_children_t2() {
        assert_eq!(
            Solution::p455_find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }

    #[test]
    fn p455_find_content_children_t3() {
        assert_eq!(
            Solution::p455_find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]),
            2
        );
    }
}
