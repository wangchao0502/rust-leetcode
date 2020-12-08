#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given a string S of digits, such as S = "123456579", we can split it into a Fibonacci-like sequence [123, 456, 579].
// Formally, a Fibonacci-like sequence is a list F of non-negative integers such that:
// 0 <= F[i] <= 2^31 - 1, (that is, each integer fits a 32-bit signed integer type);
// F.length >= 3;
// and F[i] + F[i+1] = F[i+2] for all 0 <= i < F.length - 2.
// Also, note that when splitting the string into pieces, each piece must not have extra leading zeroes,
// except if the piece is the number 0 itself.
// Return any Fibonacci-like sequence split from S, or return [] if it cannot be done.

// answers
// backtrack
impl Solution {
    fn sub_digit(chars: &[u8], start: usize, end: usize) -> i64 {
        let mut res = 0;
        for i in start..end {
            res = res * 10 + (chars[i] - b'0') as i64;
        }
        res
    }

    fn backtrack(chars: &[u8], res: &mut Vec<i32>, index: usize) -> bool {
        if index == chars.len() && res.len() >= 3 {
            return true;
        }

        for i in index..chars.len() {
            // 0 开头终止截取
            if chars[index] == b'0' && i > index {
                return false;
            }

            let num = Self::sub_digit(chars, index, i + 1);

            // 超过32位整数最大值，终止截取
            if num > i32::MAX as i64 {
                return false;
            }

            let num = num as i32;
            let len = res.len();

            if len >= 2 && num > res[len - 1] + res[len - 2] {
                return false;
            }
            if len <= 1 || num == res[len - 1] + res[len - 2] {
                res.push(num);
                // 如果找到了就直接返回
                if Self::backtrack(chars, res, i + 1) {
                    return true;
                }
                // 如果没找到，就会走回溯这一步，然后把上一步添加到集合res中的数字给移除掉
                res.pop();
            }
        }

        false
    }

    pub fn p842_split_into_fibonacci(s: String) -> Vec<i32> {
        // codes
        let chars = s.into_bytes();
        let mut ans = vec![];
        Self::backtrack(&chars, &mut ans, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p842_split_into_fibonacci_t1() {
        assert_eq!(
            Solution::p842_split_into_fibonacci("123456579".to_string()),
            vec![123, 456, 579]
        );
    }

    #[test]
    fn p842_split_into_fibonacci_t2() {
        assert_eq!(
            Solution::p842_split_into_fibonacci("112358130".to_string()),
            vec![]
        );
    }
}
