#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// hashmap -> a fixed length array
impl Solution {
    fn count(s: &str) -> [u8; 26] {
        s.chars().map(|ch| (ch as usize) - 97).fold([0u8; 26], |mut counts, ch| { counts[ch] += 1; counts })
    }

    pub fn p1002_common_chars(a: Vec<String>) -> Vec<String> {
        // codes
        let mut answer = Self::count(&a[0]);

        for s in a[1..].iter() {
            let c = Self::count(s);
            for (idx, cnt) in answer.iter_mut().enumerate() {
                *cnt = std::cmp::min(*cnt, c[idx]);
            }
        }

        let mut result = vec![];

        for idx in 0..26 {
            for _ in 0..answer[idx] {
                result.push(((idx + 97) as u8 as char).to_string());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1002_common_chars_t1() {
        assert_eq!(
            Solution::p1002_common_chars(vec![
                "bella".to_owned(),
                "label".to_owned(),
                "roller".to_owned(),
            ]),
            vec!["e".to_owned(), "l".to_owned(), "l".to_owned()]
        );
    }
}
