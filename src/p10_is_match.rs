#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
// '.' 匹配任意单个字符
// '*' 匹配零个或多个前面的那一个元素
// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。

// answers
// dp
impl Solution {
    pub fn p10_is_match(s: String, p: String) -> bool {
        // codes
        // dp[i][j]表示s的第i位，是否匹配p的j位
        let p_chars: Vec<char> = p.chars().collect();
        let s_chars: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; p_chars.len() + 1]; s_chars.len() + 1];

        dp[0][0] = true;

        for i in 0..p_chars.len() {
            if p_chars[i] == '*' {
                dp[0][i + 1] = dp[0][i - 1];
            } else {
                dp[0][i + 1] = false;
            }
        }
        for i in 1..p_chars.len() + 1 {
            for j in 1..s_chars.len() + 1 {
                if p_chars[i - 1] == s_chars[j - 1] || p_chars[i - 1] == '.' {
                    dp[j][i] = dp[j - 1][i - 1];
                } else {
                    if p_chars[i - 1] == '*' {
                        if p_chars[i - 2] == s_chars[j - 1] || p_chars[i - 2] == '.' {
                            dp[j][i] = dp[j - 1][i] || dp[j][i - 2];
                        } else {
                            dp[j][i] = dp[j][i - 2];
                        }
                    } else {
                        continue;
                    }
                }
            }
        }

        dp[s_chars.len()][p_chars.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p10_is_match_t1() {
        assert_eq!(
            Solution::p10_is_match("aa".to_string(), "a".to_string()),
            false
        );
    }

    #[test]
    fn p10_is_match_t2() {
        assert_eq!(
            Solution::p10_is_match("aa".to_string(), "a*".to_string()),
            true
        );
    }

    #[test]
    fn p10_is_match_t3() {
        assert_eq!(
            Solution::p10_is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
    }

    #[test]
    fn p10_is_match_t4() {
        assert_eq!(
            Solution::p10_is_match("ab".to_string(), ".*".to_string()),
            true
        );
    }
}
