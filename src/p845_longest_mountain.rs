#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// two pointer, special condition
impl Solution {
    pub fn p845_longest_mountain(a: Vec<i32>) -> i32 {
        // codes
        let mut left = -1;
        let mut max = 0;
        let mut is_down = false;

        for i in 1..a.len() {
            let diff = a[i] - a[i - 1];

            if diff == 0 {
                left = -1;
                is_down = false;
                continue;
            }
            if diff > 0 && left == -1 {
                left = (i - 1) as i32;
            } else if diff < 0 && left != -1 {
                max = max.max(i as i32 - left + 1);
                is_down = true;
            } else if diff > 0 && left != -1 && is_down {
                max = max.max(i as i32 - left);
                left = i as i32 - 1;
                is_down = false;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p845_longest_mountain_t1() {
        assert_eq!(
            Solution::p845_longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]),
            5
        );
    }

    #[test]
    fn p845_longest_mountain_t2() {
        assert_eq!(Solution::p845_longest_mountain(vec![2, 2, 2]), 0);
    }
    #[test]
    fn p845_longest_mountain_t3() {
        assert_eq!(
            Solution::p845_longest_mountain(vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0]),
            11
        );
    }

    #[test]
    fn p845_longest_mountain_t4() {
        assert_eq!(Solution::p845_longest_mountain(vec![2, 3, 3, 2, 0, 2]), 0);
    }

    #[test]
    fn p845_longest_mountain_t5() {
        assert_eq!(
            Solution::p845_longest_mountain(vec![875, 884, 239, 731, 723, 685]),
            4
        );
    }
}
