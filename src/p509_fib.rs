#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p509_fib(n: i32) -> i32 {
        // codes
        if n < 2 {
            return n;
        }

        let mut prepre = 0;
        let mut pre = 1;

        for _ in 3..=n {
            let cur = pre + prepre;
            prepre = pre;
            pre = cur;
        }

        pre + prepre
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
