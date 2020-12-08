#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// You're given strings J representing the types of stones that are jewels, and S representing the stones you have.
// Each character in S is a type of stone you have.  You want to know how many of the stones you have are also jewels.
// The letters in J are guaranteed distinct, and all characters in J and S are letters.
// Letters are case sensitive, so "a" is considered a different type of stone from "A".

// answers
impl Solution {
    pub fn p771_num_jewels_in_stones(j: String, s: String) -> i32 {
        // codes
        let mut rec = vec![0; 256];
        let mut ans = 0;

        for bt in s.into_bytes() {
            rec[bt as usize] += 1;
        }
        for bt in j.into_bytes() {
            ans += rec[bt as usize];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p771_num_jewels_in_stones_t1() {
        assert_eq!(
            Solution::p771_num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
    }
}
