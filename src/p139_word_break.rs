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

    #[test]
    fn p139_word_break_t1() {
        assert_eq!(
            Solution::p139_word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
    }

    #[test]
    fn p139_word_break_t2() {
        assert_eq!(
            Solution::p139_word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
    }

    #[test]
    fn p139_word_break_t3() {
        assert_eq!(
            Solution::p139_word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }
}
