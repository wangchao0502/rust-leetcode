#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给定一个非负整数 N，找出小于或等于 N 的最大的整数，同时这个整数需要满足其各个位数上的数字是单调递增。
// 当且仅当每个相邻位数上的数字 x 和 y 满足 x <= y 时，我们称这个整数是单调递增的。

// answers
impl Solution {
    pub fn p738_monotone_increasing_digits(n: i32) -> i32 {
        // codes
        // if n < 10 {
        //     return n;
        // }

        // let mut arr = vec![];
        // let mut x = n;
        // let mut stack = vec![];

        // while x > 0 {
        //     arr.push(x % 10);
        //     x /= 10;
        // }

        // let mut i = arr.len() - 1;
        // stack.push(arr[i]);

        // while i > 0 {
        //     if arr[i] > arr[i - 1] {
        //         break;
        //     }
        //     stack.push(arr[i - 1]);
        //     i -= 1;
        // }

        // if stack.len() == arr.len() {
        //     return n;
        // }

        // let mut last = stack.len() - 1;

        // while last >= 1 && stack[last] == stack[last - 1] {
        //     last -= 1;
        // }

        // let mut ans = 0;
        // let mut base = 10_i32.pow(arr.len() as u32 - 1);

        // for i in 0..=last {
        //     ans += base * stack[i];
        //     base /= 10;
        // }

        // ans - 1
        let mut ones = 111111111;
        let mut result = 0;
        for _ in 0..9 {
            while result + ones > n {
                ones /= 10;
            }
            result += ones;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p738_monotone_increasing_digits_t1() {
        assert_eq!(Solution::p738_monotone_increasing_digits(10), 9);
    }

    #[test]
    fn p738_monotone_increasing_digits_t2() {
        assert_eq!(Solution::p738_monotone_increasing_digits(1234), 1234);
    }

    #[test]
    fn p738_monotone_increasing_digits_t3() {
        assert_eq!(Solution::p738_monotone_increasing_digits(332), 299);
    }

    #[test]
    fn p738_monotone_increasing_digits_t4() {
        assert_eq!(
            Solution::p738_monotone_increasing_digits(954508339),
            899999999
        );
    }
    #[test]
    fn p738_monotone_increasing_digits_t5() {
        assert_eq!(Solution::p738_monotone_increasing_digits(668841), 667999);
    }
}
