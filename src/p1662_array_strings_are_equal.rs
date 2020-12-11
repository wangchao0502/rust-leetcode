#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
// A string is represented by an array if the array elements concatenated in order forms the string.

// answers
impl Solution {
    pub fn p1662_array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        // codes
        word1.join("") == word2.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p1662_array_strings_are_equal_t1() {
        assert_eq!(
            Solution::p1662_array_strings_are_equal(vec_string!["ab", "c"], vec_string!["a", "bc"]),
            true
        );
    }
}
