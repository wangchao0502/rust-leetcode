#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p509_fib(n: i32) -> i32 {
        // codes
        let n = n as usize;
        let mut dp = vec![0; (n + 1).max(3)];

        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;

        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p509_fib_t1() {
        assert_eq!(Solution::p509_fib(2), 1);
    }

    #[test]
    fn p509_fib_t2() {
        assert_eq!(Solution::p509_fib(4), 3);
    }
}
