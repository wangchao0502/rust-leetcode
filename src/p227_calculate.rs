#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given a string s which represents an expression, evaluate this expression and return its value.
// The integer division should truncate toward zero.

// answers
impl Solution {
    pub fn p227_calculate(s: String) -> i32 {
        // codes
        let mut stack = vec![];
        let mut num = 0;
        let mut sign = '+';
        let op = vec!['+', '-', '*', '/'];
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i];
            if c.is_numeric() {
                num = num * 10 + (c as u8 - b'0') as i32;
            }
            if op.contains(&c) || i == s.len() - 1 {
                match sign {
                    '+' => {
                        stack.push(num);
                    }
                    '-' => {
                        stack.push(-num);
                    }
                    '*' => {
                        let last = stack.pop().unwrap();
                        stack.push(last * num);
                    }
                    '/' => {
                        let last = stack.pop().unwrap();
                        stack.push(last / num);
                    }
                    _ => {}
                }
                num = 0;
                sign = c;
            }
        }

        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p227_calculate_t1() {
        assert_eq!(Solution::p227_calculate("3+2*2".to_string()), 7);
    }
}
