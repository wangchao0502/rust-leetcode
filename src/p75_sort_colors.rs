#![allow(dead_code)]

pub struct Solution {}

// two pointer
impl Solution {
    pub fn p75_sort_colors(nums: &mut Vec<i32>) {
        // all usize type
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
                    if p2 == 0 {
                        return;
                    }
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
    fn p75_sort_colors_t1() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        let output = vec![0, 0, 1, 1, 2, 2];

        Solution::p75_sort_colors(&mut input);
        assert_eq!(output, input);
    }

    #[test]
    fn p75_sort_colors_t2() {
        let mut input = vec![2];
        let output = vec![2];

        Solution::p75_sort_colors(&mut input);
        assert_eq!(output, input);
    }
}
