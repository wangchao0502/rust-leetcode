#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

// answers
impl Solution {
    pub fn p7_reverse(x: i32) -> i32 {
        // codes
        let mut ans: i64 = 0;
        let mut x = x;

        while x != 0 {
            let pop = x % 10;
            ans = ans * 10 + pop as i64;
            x /= 10;
        }

        if ans > i32::MAX as i64 || ans < i32::MIN as i64 {
            0
        } else {
            ans as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p7_reverse_t1() {
        assert_eq!(Solution::p7_reverse(123), 321);
    }

    #[test]
    fn p7_reverse_t2() {
        assert_eq!(Solution::p7_reverse(-123), -321);
    }

    #[test]
    fn p7_reverse_t3() {
        assert_eq!(Solution::p7_reverse(120), 21);
    }
}
