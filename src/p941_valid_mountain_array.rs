#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// Given an array A of integers, return true if and only if it is a valid mountain array.
// Recall that A is a mountain array if and only if:
//
// A.length >= 3
// There exists some i with 0 < i < A.length - 1 such that:
// A[0] < A[1] < ... A[i-1] < A[i]
// A[i] > A[i+1] > ... > A[A.length - 1]

// answers
impl Solution {
    pub fn p941_valid_mountain_array(a: Vec<i32>) -> bool {
        // codes
        if a.len() < 3 {
            return false;
        }

        let mut i = 0;
        let mut j = a.len() - 1;

        while i < a.len() - 1 && a[i + 1] > a[i] {
            i += 1;
        }
        while j > 0 && a[j - 1] > a[j] {
            j -= 1;
        }

        i >= j && i != a.len() - 1 && j != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p941_valid_mountain_array_t1() {
        assert_eq!(Solution::p941_valid_mountain_array(vec![2, 1]), false);
    }

    #[test]
    fn p941_valid_mountain_array_t2() {
        assert_eq!(Solution::p941_valid_mountain_array(vec![0, 3, 2, 1]), true);
    }

    #[test]
    fn p941_valid_mountain_array_t3() {
        assert_eq!(Solution::p941_valid_mountain_array(vec![0, 3, 2, 3]), false);
    }

    #[test]
    fn p941_valid_mountain_array_t4() {
        assert_eq!(Solution::p941_valid_mountain_array(vec![0, 3, 3, 1]), false);
    }

    #[test]
    fn p941_valid_mountain_array_t5() {
        assert_eq!(
            Solution::p941_valid_mountain_array(vec![0, 1, 2, 3, 4, 8, 9, 10, 11, 12, 11]),
            true
        );
    }

    #[test]
    fn p941_valid_mountain_array_t6() {
        assert_eq!(
            Solution::p941_valid_mountain_array(vec![0, 1, 2, 3, 4, 8, 9]),
            false
        );
    }
}
