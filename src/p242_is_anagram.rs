#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given two strings s and t , write a function to determine if t is an anagram of s.

// answers
// 异构词，排序并比较
impl Solution {
    pub fn p242_is_anagram(s: String, t: String) -> bool {
        // codes
        let (mut s, mut t) = (s.into_bytes(), t.into_bytes());
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p242_is_anagram_t1() {
        assert_eq!(
            Solution::p242_is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }
}
