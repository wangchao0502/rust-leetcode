#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一种规律 pattern 和一个字符串 str ，判断 str 是否遵循相同的规律。
// 这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 str 中的每个非空单词之间存在着双向连接的对应规律。

// answers
impl Solution {
    pub fn p290_word_pattern(pattern: String, s: String) -> bool {
        // codes
        use std::collections::HashMap;
        let words = s.split(' ').collect::<Vec<&str>>();

        if pattern.len() != words.len() {
            return false;
        }

        let mut word_map = HashMap::new();
        let mut pattern_map = HashMap::new();

        for (i, c) in pattern.chars().enumerate() {
            let target_word = pattern_map.entry(c).or_insert(words[i]);
            let target_pattern = word_map.entry(words[i]).or_insert(c);

            if *target_pattern != c || *target_word != words[i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p290_word_pattern_t1() {
        assert_eq!(
            Solution::p290_word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
    }

    #[test]
    fn p290_word_pattern_t2() {
        assert_eq!(
            Solution::p290_word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }

    #[test]
    fn p290_word_pattern_t3() {
        assert_eq!(
            Solution::p290_word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
    }

    #[test]
    fn p290_word_pattern_t4() {
        assert_eq!(
            Solution::p290_word_pattern("aaa".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }
}
