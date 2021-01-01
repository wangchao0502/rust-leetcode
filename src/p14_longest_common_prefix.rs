#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 编写一个函数来查找字符串数组中的最长公共前缀。
// 如果不存在公共前缀，返回空字符串 ""。

// answers
impl Solution {
    pub fn p14_longest_common_prefix(strs: Vec<String>) -> String {
        // codes
        if strs.len() == 0 || strs[0] == "" {
            return "".to_string();
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }
        let mut result = String::from("");
        let mut temp: u8 = 0;
        let mut ptr = 0;

        loop {
            for i in 0..strs.len() {
                if ptr >= strs[i].len() {
                    return result.to_string();
                } else if i == 0 {
                    temp = strs[i].as_bytes()[ptr];
                } else if strs[i].as_bytes()[ptr] != temp {
                    return result.to_string();
                }
            }
            let str = strs[0].as_bytes()[ptr];
            result.push(str as char);
            ptr += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p14_longest_common_prefix_t1() {
        assert_eq!(
            Solution::p14_longest_common_prefix(vec_string!["flower", "flow", "flight"]),
            "fl".to_string()
        );
    }
}
