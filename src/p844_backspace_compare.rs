#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// stack
impl Solution {
    fn parser(s: String) -> String {
        let mut result = String::new();
        for c in s.chars() {
            if c == '#' {
                result.pop();
            } else {
                result.push(c);
            }
        }
        result
    }

    pub fn p844_backspace_compare(s: String, t: String) -> bool {
        // codes
        Self::parser(s) == Self::parser(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p844_backspace_compare_t1() {
        assert_eq!(
            Solution::p844_backspace_compare(
                "ab#c".to_string(),
                "ad#c".to_string()
            ),
            true
        );
    }
}

