#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定两个字符串 s 和 t，它们只包含小写字母。
// 字符串 t 由字符串 s 随机重排，然后在随机位置添加一个字母。
// 请找出在 t 中被添加的字母。

// answers
// 位运算
impl Solution {
    pub fn p389_find_the_difference(s: String, t: String) -> char {
        // codes
        let mut ans: u8 = 0;

        for c in s.into_bytes() {
            ans ^= c;
        }

        for c in t.into_bytes() {
            ans ^= c;
        }

        ans as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p389_find_the_difference_t1() {
        assert_eq!(
            Solution::p389_find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
}
