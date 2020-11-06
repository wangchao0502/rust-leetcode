#![allow(dead_code)]

// use mods

pub struct Solution {}

// Given an integer array arr. You have to sort the integers in the array in ascending order
// by the number of 1's in their binary representation and in case of two or more integers
// have the same number of 1's you have to sort them in ascending order.
// Return the sorted array.

// answers
impl Solution {
    fn count_one(i: i32) -> i32 {
        let mut count = 0;
        let mut i = i;

        while i > 0 {
            count += i & 1;
            i >>= 1;
        }

        count
    }

    pub fn p1356_sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        // codes
        let mut ans = vec![];
        for i in arr {
            ans.push([Self::count_one(i), i]);
        }

        ans.sort();
        ans.iter().map(|&x| x[1]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1356_sort_by_bits_t1() {
        assert_eq!(
            Solution::p1356_sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
    }

    #[test]
    fn p1356_sort_by_bits_t2() {
        assert_eq!(
            Solution::p1356_sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }

    #[test]
    fn p1356_sort_by_bits_t3() {
        assert_eq!(
            Solution::p1356_sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }

    #[test]
    fn p1356_sort_by_bits_t4() {
        assert_eq!(
            Solution::p1356_sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]),
            vec![2, 3, 5, 17, 7, 11, 13, 19]
        );
    }
}
