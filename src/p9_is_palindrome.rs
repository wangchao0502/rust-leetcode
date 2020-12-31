#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p9_is_palindrome(mut x: i32) -> bool {
        // codes
        // 负数和结尾是0的多位正数
        if x < 0 || (x % 10 == 0 && x / 10 != 0) {
            return false;
        }

        let mut y = 0;

        // 121  => x:1, y: 12
        // 1221 => x:12, y: 12
        while y < x {
            y = y * 10 + x % 10;
            x /= 10;
        }

        x == y || x == y / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p9_is_palindrome_t1() {
        assert_eq!(Solution::p9_is_palindrome(121), true);
    }

    #[test]
    fn p9_is_palindrome_t2() {
        assert_eq!(Solution::p9_is_palindrome(-121), false);
    }

    #[test]
    fn p9_is_palindrome_t3() {
        assert_eq!(Solution::p9_is_palindrome(10), false);
    }
}
