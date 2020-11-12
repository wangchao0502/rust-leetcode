#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs
// Given an array A of non-negative integers, half of the integers in A are odd, and half of the integers are even.
// Sort the array so that whenever A[i] is odd, i is odd; and whenever A[i] is even, i is even.
// You may return any answer array that satisfies this condition.

// answers
impl Solution {
    pub fn p922_sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut array = vec![0; a.len()];
        let mut o = 1;
        let mut e = 0;
        for i in a {
            if i & 1 == 0 {
                array[e] = i;
                e += 2;
            } else {
                array[o] = i;
                o += 2;
            }
        }
        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p922_sort_array_by_parity_ii_t1() {
        assert_eq!(
            Solution::p922_sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            vec![4, 5, 2, 7]
        );
    }

    #[test]
    fn p922_sort_array_by_parity_ii_t2() {
        assert_eq!(
            Solution::p922_sort_array_by_parity_ii(vec![
                648, 831, 560, 986, 192, 424, 997, 829, 897, 843
            ]),
            vec![648, 831, 560, 997, 192, 897, 986, 829, 424, 843]
        );
    }
}
