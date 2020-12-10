#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Evaluate the value of an arithmetic expression in Reverse Polish Notation.
// Valid operators are +, -, *, /. Each operand may be an integer or another expression.
//
// Note:
// Division between two integers should truncate toward zero.
// The given RPN expression is always valid.
// That means the expression would always evaluate to a result and there won't be any divide by zero operation.

// answers
impl Solution {
    fn op_exec(s: &String, a: i32, b: i32) -> i32 {
        return match s.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => 0,
        };
    }

    pub fn p150_eval_rpn(tokens: Vec<String>) -> i32 {
        // codes
        let mut stack = vec![];
        let op = vec!["+", "-", "*", "/"];

        for s in tokens {
            if op.contains(&s.as_str()) {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Self::op_exec(&s, b, a));
            } else {
                stack.push(s.parse::<i32>().unwrap());
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p150_eval_rpn_t1() {
        // ((2 + 1) * 3) = 9
        assert_eq!(
            Solution::p150_eval_rpn(vec_string!["2", "1", "+", "3", "*"]),
            9
        );
    }
    #[test]
    fn p150_eval_rpn_t2() {
        // ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
        assert_eq!(
            Solution::p150_eval_rpn(vec_string![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]),
            22
        );
    }
}
