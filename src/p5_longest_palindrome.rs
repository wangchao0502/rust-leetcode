#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

// answers
impl Solution {
    pub fn p5_longest_palindrome(s: String) -> String {
        // codes
        let n = s.len();
        // dp[i][j]从i-j是否为回文数
        // dp[i][j] = dp[i] == dp[j] && dp[i + 1][j - 1]
        // j - i < 3 dp[i][j] = dp[i] == dp[j]
        // dp[i][i] = true
        let chars = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![false; n]; n];
        let mut max_len = 1;
        let mut begin = 0;

        for i in 0..n {
            dp[i][i] = true;
        }

        for j in 1..n {
            for i in 0..j {
                if chars[i] == chars[j] {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                } else {
                    dp[i][j] = false;
                }
                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    begin = i;
                }
            }
        }

        String::from(&s[begin..begin + max_len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5_longest_palindrome_t1() {
        assert_eq!(
            Solution::p5_longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn p5_longest_palindrome_t2() {
        assert_eq!(
            Solution::p5_longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn p5_longest_palindrome_t3() {
        assert_eq!(
            Solution::p5_longest_palindrome("ac".to_string()),
            "a".to_string()
        );
    }
}
