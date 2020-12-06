#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// You own a Goal Parser that can interpret a string command.
// The command consists of an alphabet of "G", "()" and/or "(al)" in some order.
// The Goal Parser will interpret "G" as the string "G", "()" as the string "o",
// and "(al)" as the string "al". The interpreted strings are then concatenated in the original order.
// Given the string command, return the Goal Parser's interpretation of command.

// answers
impl Solution {
    pub fn p5617_interpret(command: String) -> String {
        // codes
        let mut ans = vec![];
        let mut i = 0;
        let chars: Vec<char> = command.chars().collect();

        while i < chars.len() {
            if chars[i] == '(' {
                i += 1;
                if chars[i] == ')' {
                    ans.push('o');
                } else {
                    ans.push(chars[i]);
                }
            } else if chars[i] != ')' {
                ans.push(chars[i]);
            }
            i += 1;
        }

        ans.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5617_interpret_t1() {
        assert_eq!(
            Solution::p5617_interpret("G()(al)".to_string()),
            "Goal".to_string()
        );
    }

    #[test]
    fn p5617_interpret_t2() {
        assert_eq!(
            Solution::p5617_interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
    }
}
