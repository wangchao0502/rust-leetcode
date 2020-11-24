#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// Given a string s. You should re-order the string using the following algorithm:
//
// 1. Pick the smallest character from s and append it to the result.
// 2. Pick the smallest character from s which is greater than the last appended character to the result and append it.
// 3. Repeat step 2 until you cannot pick more characters.
// 4. Pick the largest character from s and append it to the result.
// 5. Pick the largest character from s which is smaller than the last appended character to the result and append it.
// 6. Repeat step 5 until you cannot pick more characters.
// 7. Repeat the steps from 1 to 6 until you pick all characters from s.
//
// In each step, If the smallest or the largest character appears more than
// once you can choose any occurrence and append it to the result.
//
// Return the result string after sorting s with this algorithm.

// answers
impl Solution {
    pub fn p1370_sort_string(s: String) -> String {
        // codes
        let mut nums = vec![0; 26];
        let mut ans = String::new();

        for c in s.bytes() {
            nums[(c - b'a') as usize] += 1;
        }

        while ans.len() < s.len() {
            for i in 0..26 {
                if nums[i] != 0 {
                    ans.push((i as u8 + b'a') as char);
                    nums[i] -= 1;
                }
            }

            for i in (0..26).rev() {
                if nums[i] != 0 {
                    ans.push((i as u8 + b'a') as char);
                    nums[i] -= 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1370_sort_string_t1() {
        assert_eq!(
            Solution::p1370_sort_string("aaaabbbbcccc".to_string()),
            "abccbaabccba".to_string()
        );
    }

    #[test]
    fn p1370_sort_string_t2() {
        assert_eq!(
            Solution::p1370_sort_string("rat".to_string()),
            "art".to_string()
        );
    }
}
