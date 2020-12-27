#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你一个偶数长度的字符串 s 。将其拆分成长度相同的两半,前一半为 a ,后一半为 b 。
// 两个字符串 相似 的前提是它们都含有相同数目的元音（'a','e','i','o','u','A','E','I','O','U'）。
// 注意,s 可能同时含有大写和小写字母。
// 如果 a 和 b 相似,返回 true ；否则,返回 false 。

// answers
impl Solution {
    pub fn p5637_halves_are_alike(s: String) -> bool {
        // codes
        let yy = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut left_count = 0;
        let mut rigth_count = 0;
        let len = s.len();

        for (i, c) in s.chars().enumerate() {
            if yy.contains(&c) {
                if i < len / 2 {
                    left_count += 1;
                } else {
                    rigth_count += 1;
                }
            }
        }

        left_count == rigth_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5637_halves_are_alike_t1() {
        assert_eq!(Solution::p5637_halves_are_alike("book".to_string()), true);
    }

    #[test]
    fn p5637_halves_are_alike_t2() {
        assert_eq!(
            Solution::p5637_halves_are_alike("textbook".to_string()),
            false
        );
    }
}
