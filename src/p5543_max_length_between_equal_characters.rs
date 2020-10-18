#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p5543_max_length_between_equal_characters(s: String) -> i32 {
        // codes
        let mut record = vec![-1 as i32; 26];
        let mut result = -1;

        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 97;
            if record[idx] == -1 {
                record[idx] = i as i32;
            } else {
                result = result.max(i as i32 - record[idx] - 1);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5543_max_length_between_equal_characters_t1() {
        assert_eq!(
            Solution::p5543_max_length_between_equal_characters("aa".to_string()),
            0
        );
    }

    #[test]
    fn p5543_max_length_between_equal_characters_t2() {
        assert_eq!(
            Solution::p5543_max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }

    #[test]
    fn p5543_max_length_between_equal_characters_t3() {
        assert_eq!(
            Solution::p5543_max_length_between_equal_characters("cabbac".to_string()),
            4
        );
    }
}
