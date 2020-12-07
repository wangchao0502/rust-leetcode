#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.

// answers
impl Solution {
    pub fn p20_is_valid(s: String) -> bool {
        // codes
        let mut stack = vec![];

        for c in s.chars() {
            if c == '(' {
                stack.push(')');
            } else if c == '{' {
                stack.push('}');
            } else if c == '[' {
                stack.push(']');
            } else if stack.is_empty() || c != stack.pop().unwrap_or(' ') {
                return false;
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p20_is_valid_t1() {
        assert_eq!(Solution::p20_is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn p20_is_valid_t2() {
        assert_eq!(Solution::p20_is_valid("(]{}".to_string()), false);
    }

    #[test]
    fn p20_is_valid_t3() {
        assert_eq!(Solution::p20_is_valid("{[()]}".to_string()), true);
    }
}
