#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。
// 需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。

// answers
// 单调栈
impl Solution {
    pub fn p316_remove_duplicate_letters(s: String) -> String {
        // codes
        let mut ans = vec![];
        let mut d = [false; 26];
        for (i, c) in s.bytes().enumerate() {
            if !d[c as usize - 97] {
                let t = s[i..].as_bytes();
                while !ans.is_empty() && ans[ans.len() - 1] > c && t.contains(&ans[ans.len() - 1]) {
                    d[ans.pop().unwrap() as usize - 97] = false;
                }
                ans.push(c);
                d[c as usize - 97] = true
            }
        }
        String::from_utf8(ans).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p316_remove_duplicate_letters_t1() {
        assert_eq!(
            Solution::p316_remove_duplicate_letters("bcabc".to_string()),
            "abc".to_string()
        );
    }
}
