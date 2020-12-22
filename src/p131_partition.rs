#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个字符串 s，将 s 分割成一些子串，使每个子串都是回文串。
// 返回 s 所有可能的分割方案。

// answers
// backtracking
impl Solution {
    // 这里可以加一个HashMap保存历史结果
    fn is_palindrome(s: &str) -> bool {
        let mut i = 0;

        while i <= (s.len() - 1) / 2 {
            if s.as_bytes()[i] != s.as_bytes()[s.len() - i - 1] {
                return false;
            }
            i += 1;
        }

        true
    }

    fn backtracking(s: &str, idx: usize, ans: &mut Vec<Vec<String>>, path: &mut Vec<String>) {
        if idx >= s.len() {
            ans.push(path.clone());
            return;
        }
        for i in idx..s.len() {
            if Self::is_palindrome(&s[idx..=i]) {
                let str = String::from(&s[idx..=i]);
                path.push(str);
            } else {
                continue;
            }
            Self::backtracking(s, i + 1, ans, path);
            path.pop();
        }
    }

    pub fn p131_partition(s: String) -> Vec<Vec<String>> {
        // codes
        let mut ans = vec![];
        let mut path = vec![];
        Self::backtracking(&s[..], 0, &mut ans, &mut path);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::{assert_eq_sorted, vec_string};

    #[test]
    fn p131_partition_t1() {
        assert_eq_sorted!(
            Solution::p131_partition("aab".to_string()),
            vec![vec_string!["aa", "b"], vec_string!["a", "a", "b"]]
        );
    }
}
