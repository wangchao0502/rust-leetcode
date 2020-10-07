#![allow(dead_code)]

pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return
        }

        let (mut p0, mut p1, mut p2) = (0, 0, nums.len() - 1);

        while p1 <= p2 {
            match nums[p1] {
                0 => {
                    nums.swap(p0, p1);
                    p0 += 1;
                    p1 += 1;
                }
                2 => {
                    nums.swap(p1, p2);
                    p2 -= 1;
                }
                _ => p1 += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        let output = vec![0, 0, 1, 1, 2, 2];

        Solution::sort_colors(&mut input);
        assert_eq!(output, input);
    }

    #[test]
    fn t2() {
        let mut input = vec![2];
        let output = vec![2];

        Solution::sort_colors(&mut input);
        assert_eq!(output, input);
    }
}
