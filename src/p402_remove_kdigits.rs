#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
// 单调栈
impl Solution {
    pub fn p402_remove_kdigits(num: String, k: i32) -> String {
        // codes
        let mut k = k as usize;
        let mut digits = String::with_capacity(num.len()); // Don't grow `digits` over time, just allocate memory once.
        for digit in num.chars() {
            while k > 0 && !digits.is_empty() && digit < digits.chars().last().unwrap() {
                digits.pop(); // Remove digits larger than the current one.
                k -= 1;
            }
            if digits.is_empty() && digit == '0' {
                continue; // Skip leading zeros.
            }
            digits.push(digit);
        }
        // By construction, the right-most digits are the larger ones, therefore
        // if digits still need to be removed (i.e. k > 0), we pop k digits from
        // the right end of the `digits` array until we have removed enough.
        for _ in 0..k {
            digits.pop();
        }
        if digits.is_empty() {
            String::from("0")
        } else {
            digits
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p402_remove_kdigits_t1() {
        assert_eq!(
            Solution::p402_remove_kdigits("1432219".to_string(), 3),
            "1219".to_string()
        );
    }
}
