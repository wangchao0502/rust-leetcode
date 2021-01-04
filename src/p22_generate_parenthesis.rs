#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

// answers
impl Solution {
    fn backtracking(s: String, n: usize, right: usize, left: usize, result: &mut Vec<String>) {
        if s.len() == 2 * n {
            result.push(s);
            return;
        }

        if left < n {
            let mut s = s.to_owned();
            s.push('(');
            Self::backtracking(s, n, right, left + 1, result);
        }

        if left > 0 && left > right && right < n {
            let mut s = s.to_owned();
            s.push(')');
            Self::backtracking(s, n, right + 1, left, result);
        }
    }

    pub fn p22_generate_parenthesis(n: i32) -> Vec<String> {
        // codes
        let mut ans = vec![];
        Self::backtracking(String::new(), n as usize, 0, 0, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::{assert_eq_sorted, vec_string};

    #[test]
    fn p22_generate_parenthesis_t1() {
        assert_eq_sorted!(
            Solution::p22_generate_parenthesis(3),
            vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
