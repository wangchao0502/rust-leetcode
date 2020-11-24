#![allow(dead_code)]

// use mods
use std::collections::HashMap;

pub struct Solution {}

// add structs

// answers
// backtrack get all answers, use memo to make faster
impl Solution {
    fn backtrack(
        s: &str,
        words: &[String],
        memo: &mut HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        // &s.to_string() => &(s.to_string())
        if let Some(v) = memo.get(&s.to_string()) {
            return v.clone();
        }
        let mut ret = vec![];
        for word in words.iter() {
            if s.starts_with(word) {
                if word == s {
                    ret.push(word.clone());
                    continue;
                }
                for e in Self::backtrack(&s[word.len()..], words, memo).iter() {
                    let mut ss = word.clone();
                    ss.push(' ');
                    ret.push(ss + e);
                }
            }
        }
        memo.insert(s.to_string(), ret.clone());
        ret
    }

    pub fn p140_word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        // codes
        let mut memo: HashMap<String, Vec<String>> = HashMap::new();
        Self::backtrack(&s, &word_dict, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p140_word_break_t1() {
        assert_eq!(
            Solution::p140_word_break(
                "catsanddog".to_string(),
                vec_string!["cat", "cats", "and", "sand", "dog"]
            ),
            vec_string!["cat sand dog", "cats and dog"]
        );
    }

    #[test]
    fn p140_word_break_t2() {
        assert_eq!(
            Solution::p140_word_break(
                "pineapplepenapple".to_string(),
                vec_string!["apple", "pen", "applepen", "pine", "pineapple"]
            ),
            vec_string![
                "pine apple pen apple",
                "pine applepen apple",
                "pineapple pen apple"
            ]
        );
    }
}
