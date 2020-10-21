#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// two pointers
impl Solution {
    pub fn p925_is_long_pressed_name(name: String, typed: String) -> bool {
        // codes
        let mut i = 0;
        let mut j = 0;

        let name: Vec<u8> = name.bytes().collect();
        let typed: Vec<u8> = typed.bytes().collect();

        while i < name.len() {
            if j >= typed.len() {
                return false;
            }

            if name[i] == typed[j] {
                i += 1;
                j += 1;
            } else if j > 0 && typed[j - 1] == typed[j] {
                j += 1;
            } else {
                return false;
            }
        }

        // check typed extra chars
        while j < typed.len() {
            if typed[j] != name[i - 1] {
                return false;
            }
            j += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p925_is_long_pressed_name_t1() {
        assert_eq!(
            Solution::p925_is_long_pressed_name("alex".to_string(), "aaleex".to_string()),
            true
        );
    }
    #[test]
    fn p925_is_long_pressed_name_t2() {
        assert_eq!(
            Solution::p925_is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
            false
        );
    }

    #[test]
    fn p925_is_long_pressed_name_t3() {
        assert_eq!(
            Solution::p925_is_long_pressed_name("pyplrz".to_string(), "ppyypllr".to_string()),
            false
        );
    }

    #[test]
    fn p925_is_long_pressed_name_t4() {
        assert_eq!(
            Solution::p925_is_long_pressed_name("alex".to_string(), "alexxr".to_string()),
            false
        );
    }
}
