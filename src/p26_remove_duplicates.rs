#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// two pointer
impl Solution {
    pub fn p26_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // codes
        let mut p1: usize = 0;
        let mut p2: usize = 0;

        while p2 < nums.len() {
            if nums[p1] != nums[p2] {
                nums[p1 + 1] = nums[p2];
                p1 += 1;
            }
            p2 += 1;
        }

        (p1 + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p26_remove_duplicates_t1() {
        assert_eq!(Solution::p26_remove_duplicates(&mut vec![1, 1, 2, 3]), 3);
    }

    #[test]
    fn p26_remove_duplicates_t2() {
        assert_eq!(
            Solution::p26_remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }

    #[test]
    fn p26_remove_duplicates_t3() {
        assert_eq!(Solution::p26_remove_duplicates(&mut vec![]), 0);
    }
}
