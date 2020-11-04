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

    #[test]
    fn p140_word_break_t1() {
        assert_eq!(
            Solution::p140_word_break(
                "catsanddog".to_string(),
                vec![
                    "cat".to_string(),
                    "cats".to_string(),
                    "and".to_string(),
                    "sand".to_string(),
                    "dog".to_string(),
                ]
            ),
            vec!["cat sand dog".to_string(), "cats and dog".to_string()]
        );
    }

    #[test]
    fn p140_word_break_t2() {
        assert_eq!(
            Solution::p140_word_break(
                "pineapplepenapple".to_string(),
                vec![
                    "apple".to_string(),
                    "pen".to_string(),
                    "applepen".to_string(),
                    "pine".to_string(),
                    "pineapple".to_string()
                ]
            ),
            vec![
                "pine apple pen apple".to_string(),
                "pine applepen apple".to_string(),
                "pineapple pen apple".to_string(),
            ]
        );
    }
}
