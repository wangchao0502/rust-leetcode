#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个字符串 s，将 s 分割成一些子串，使每个子串都是回文串。
// 返回 s 所有可能的分割方案。

// class Solution {
//     private:
//         vector<vector<string>> result;
//         vector<string> path; // 放已经回文的子串
//         void backtracking (const string& s, int startIndex) {
//             if (startIndex >= s.size()) {
//                 result.push_back(path);
//                 return;
//             }
//             for (int i = startIndex; i < s.size(); i++) {
//                 if (isPalindrome(s, startIndex, i)) {   // 是回文子串
//                     string str = s.substr(startIndex, i - startIndex + 1);
//                     path.push_back(str);
//                 } else {                                // 不是回文，跳过
//                     continue;
//                 }
//                 backtracking(s, i + 1); // 寻找i+1为起始位置的子串
//                 path.pop_back(); // 回溯过程，弹出本次已经填在的子串
//             }
//         }
//         bool isPalindrome(const string& s, int start, int end) {
//             for (int i = start, j = end; i < j; i++, j--) {
//                 if (s[i] != s[j]) {
//                     return false;
//                 }
//             }
//             return true;
//         }
//     public:
//         vector<vector<string>> partition(string s) {
//             result.clear();
//             path.clear();
//             backtracking(s, 0);
//             return result;
//         }
//     };

// answers
// backtracking
impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let mut i = 0;
        
        while i <= (s.len() - 1) / 2 {
            if s.as_bytes()[i] != s.as_bytes()[s.len() - i - 1] {
                return false;
            }
            i += 1;
        }

        true
    }

    fn backtracking(s: &str, idx: usize, ans: &mut Vec<Vec<String>>, path: &mut Vec<String>) {
        if idx >= s.len() {
            ans.push(path.clone());
            return;
        }
        for i in idx..s.len() {
            if Self::is_palindrome(&s[idx..=i]) {
                let str = String::from(&s[idx..=i]);
                path.push(str);
            } else {
                continue;
            }
            Self::backtracking(s, i + 1, ans, path);
            path.pop();
        }
    }

    pub fn p131_partition(s: String) -> Vec<Vec<String>> {
        // codes
        let mut ans = vec![];
        let mut path = vec![];
        Self::backtracking(&s[..], 0, &mut ans, &mut path);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::{vec_string, assert_eq_sorted};

    #[test]
    fn p131_partition_t1() {
        assert_eq_sorted!(
            Solution::p131_partition("aab".to_string()),
            vec![vec_string!["aa", "b"], vec_string!["a", "a", "b"]]
        );
    }
}
