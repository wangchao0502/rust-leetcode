#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p31_next_permutation(nums: &mut Vec<i32>) {
        // codes
        let mut p1 = nums.len() - 1;
        let mut p2 = nums.len() - 1;
        let mut p3 = nums.len() - 1;
        while p2 > 0 {
            if nums[p2 - 1] < nums[p2] {
                break;
            }
            p2 -= 1;
        }

        // find p1
        if p2 != 0 {
            while p1 > 0 {
                if nums[p1] > nums[p2 - 1] {
                    break;
                }
                p1 -= 1;
            }
            nums.swap(p2 - 1, p1);
        }

        while p3 > p2 {
            nums.swap(p2, p3);
            p3 -= 1;
            p2 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p31_next_permutation_t1() {
        let mut input = vec![1, 2, 3];
        Solution::p31_next_permutation(&mut input);
        assert_eq!(input, vec![1, 3, 2]);
    }

    #[test]
    fn p31_next_permutation_t2() {
        let mut input = vec![1, 5, 8, 4, 7, 6, 5, 3, 1];
        Solution::p31_next_permutation(&mut input);
        assert_eq!(input, vec![1, 5, 8, 5, 1, 3, 4, 6, 7]);
    }

    #[test]
    fn p31_next_permutation_t3() {
        let mut input = vec![3, 2, 1];
        Solution::p31_next_permutation(&mut input);
        assert_eq!(input, vec![1, 2, 3]);
    }

    #[test]
    fn p31_next_permutation_t4() {
        let mut input = vec![1];
        Solution::p31_next_permutation(&mut input);
        assert_eq!(input, vec![1]);
    }

    #[test]
    fn p31_next_permutation_t5() {
        let mut input = vec![1, 5, 1];
        Solution::p31_next_permutation(&mut input);
        assert_eq!(input, vec![5, 1, 1]);
    }

    #[test]
    fn p31_next_permutation_t6() {
        let mut input = vec![5, 1, 1];
        Solution::p31_next_permutation(&mut input);
        assert_eq!(input, vec![1, 1, 5]);
    }
}
