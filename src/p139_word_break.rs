#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// dp
impl Solution {
    pub fn p139_word_break(s: String, word_dict: Vec<String>) -> bool {
        // codes
        let mut dp = vec![true; s.len() + 1];
        for i in 1..s.len() + 1 {
            for j in (0..i).rev() {
                dp[i] = dp[j] && word_dict.contains(&s[j..i].to_string());
                if dp[i] {
                    break;
                }
            }
        }
        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p139_word_break_t1() {
        assert_eq!(
            Solution::p139_word_break("leetcode".to_string(), vec_string!["leet", "code"]),
            true
        );
    }

    #[test]
    fn p139_word_break_t2() {
        assert_eq!(
            Solution::p139_word_break("applepenapple".to_string(), vec_string!["apple", "pen"]),
            true
        );
    }

    #[test]
    fn p139_word_break_t3() {
        assert_eq!(
            Solution::p139_word_break(
                "catsandog".to_string(),
                vec_string!["cats", "dog", "sand", "and", "cat"]
            ),
            false
        );
    }
}
