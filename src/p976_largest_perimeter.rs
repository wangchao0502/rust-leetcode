#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given an array A of positive lengths, return the largest perimeter of a triangle with non-zero area,
// formed from 3 of these lengths.
// If it is impossible to form any triangle of non-zero area, return 0.

// answers
impl Solution {
    pub fn p976_largest_perimeter(mut a: Vec<i32>) -> i32 {
        // codes
        a.sort();

        for i in (2..a.len()).rev() {
            if a[i - 2] + a[i - 1] > a[i] {
                return a[i - 2] + a[i - 1] + a[i];
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p976_largest_perimeter_t1() {
        assert_eq!(Solution::p976_largest_perimeter(vec![2, 1, 2]), 5);
    }

    #[test]
    fn p976_largest_perimeter_t2() {
        assert_eq!(Solution::p976_largest_perimeter(vec![3, 2, 3, 4]), 10);
    }

    #[test]
    fn p976_largest_perimeter_t3() {
        assert_eq!(Solution::p976_largest_perimeter(vec![1, 2, 1]), 0);
    }
}
