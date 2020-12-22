#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个由不同字符组成的字符串 allowed 和一个字符串数组 words 。
// 如果一个字符串的每一个字符都在 allowed 中，就称这个字符串是 一致字符串 。
// 请你返回 words 数组中 一致字符串 的数目。

// answers
impl Solution {
    pub fn p1684_count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        // codes
        let mut set = vec![0; 26];
        let mut ans = 0;

        for byte in allowed.into_bytes() {
            set[(byte - b'a') as usize] = 1;
        }

        for word in words {
            let mut is_allow = true;
            for byte in word.into_bytes() {
                if set[(byte - b'a') as usize] == 0 {
                    is_allow = false;
                    break;
                }
            }
            if is_allow {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p1684_count_consistent_strings_t1() {
        assert_eq!(
            Solution::p1684_count_consistent_strings(
                "ab".to_string(),
                vec_string!["ad", "bd", "aaab", "baa", "badab"]
            ),
            2
        );
    }
}
