#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个Excel表格中的列名称，返回其相应的列序号。
//
// 例如，
//
//     A -> 1
//     B -> 2
//     C -> 3
//     ...
//     Z -> 26
//     AA -> 27
//     AB -> 28
//     ...

// answers
impl Solution {
    pub fn p171_title_to_number(s: String) -> i32 {
        // codes
        let mut ans = 0;

        for byte in s.into_bytes() {
            ans = ans * 26 + (byte - b'A' + 1) as i32;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p171_title_to_number_t1() {
        assert_eq!(Solution::p171_title_to_number("A".to_string()), 1);
    }

    #[test]
    fn p171_title_to_number_t2() {
        assert_eq!(Solution::p171_title_to_number("ZY".to_string()), 701);
    }
}
