#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p164_maximum_gap(nums: Vec<i32>) -> i32 {
        // codes
        let len = nums.len();

        if len < 2 {
            return 0;
        }

        let (mut min, mut max) = (i32::MAX, i32::MIN);
        nums.iter().for_each(|&x| {
            min = min.min(x);
            max = max.max(x);
        });
        let d = 1.max((max - min) as usize / (len - 1));
        let size = 1 + (max - min) as usize / d;
        let mut buckets = vec![None; size];

        nums.into_iter().for_each(|val| {
            let b = &mut buckets[(val - min) as usize / d];
            if let Some((x, y)) = b {
                *x = val.min(*x);
                *y = val.max(*y);
            } else {
                *b = Some((val, val));
            }
        });
        buckets
            .into_iter()
            .filter_map(|x| x)
            .fold((0, min), |(res, prv), (min, max)| (res.max(min - prv), max))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p164_maximum_gap_t1() {
        assert_eq!(Solution::p164_maximum_gap(vec![3, 6, 9, 1]), 3);
    }

    #[test]
    fn p164_maximum_gap_t2() {
        assert_eq!(
            Solution::p164_maximum_gap(vec![1, 1, 1, 1, 1, 5, 5, 5, 5, 5]),
            4
        );
    }

    #[test]
    fn p164_maximum_gap_t3() {
        assert_eq!(Solution::p164_maximum_gap(vec![1, 100000]), 99999);
    }
}
