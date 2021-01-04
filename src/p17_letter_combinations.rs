#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
// 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

// answers
impl Solution {
    fn match_letters(c: char) -> Vec<char> {
        match c {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        }
    }

    fn helper(chars: &Vec<char>, index: usize, ans: &mut Vec<String>, cur: Vec<char>) {
        if index == chars.len() {
            ans.push(cur.iter().collect::<String>());
            return;
        }

        for cc in Self::match_letters(chars[index]) {
            let mut cur = cur.clone();
            cur.push(cc);
            Self::helper(&chars, index + 1, ans, cur);
        }
    }

    pub fn p17_letter_combinations(digits: String) -> Vec<String> {
        // codes
        let mut ans = vec![];

        if digits.len() == 0 {
            return ans;
        }

        let chars = digits.chars().collect::<Vec<char>>();
        Self::helper(&chars, 0, &mut ans, vec![]);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p17_letter_combinations_t1() {
        assert_eq!(
            Solution::p17_letter_combinations("23".to_string()),
            vec_string!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn p17_letter_combinations_t2() {
        assert_eq!(
            Solution::p17_letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }
}
