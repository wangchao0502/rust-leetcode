#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。

// answers
impl Solution {
    pub fn p387_first_uniq_char(s: String) -> i32 {
        // codes
        let mut count = vec![0; 26];

        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if count[c as usize - 'a' as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p387_first_uniq_char_t1() {
        assert_eq!(Solution::p387_first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn p387_first_uniq_char_t2() {
        assert_eq!(
            Solution::p387_first_uniq_char("loveleetcode".to_string()),
            2
        );
    }
}
