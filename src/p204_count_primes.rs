#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Count the number of prime numbers less than a non-negative number, n.

// answers
// 埃氏筛，线性筛
impl Solution {
    pub fn p204_count_primes(n: i32) -> i32 {
        // codes
        let mut is_prime = vec![1; n as usize];
        let mut primes = vec![];

        for i in 2..n {
            if is_prime[i as usize] == 1 {
                primes.push(i);
            }
            for j in 0..primes.len() {
                let k = i as i64 * primes[j] as i64;
                if k < n as i64 {
                    is_prime[k as usize] = 0;
                    if i % primes[j] == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        primes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p204_count_primes_t1() {
        assert_eq!(Solution::p204_count_primes(10), 4);
    }

    #[test]
    fn p204_count_primes_t2() {
        assert_eq!(Solution::p204_count_primes(0), 0);
    }

    #[test]
    fn p204_count_primes_t3() {
        assert_eq!(Solution::p204_count_primes(499979), 41537);
    }
}
