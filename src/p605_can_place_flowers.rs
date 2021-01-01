#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 假设你有一个很长的花坛，一部分地块种植了花，另一部分却没有。
// 可是，花卉不能种植在相邻的地块上，它们会争夺水源，两者都会死去。
// 给定一个花坛（表示为一个数组包含0和1，其中0表示没种植花，1表示种植了花），和一个数 n 。
// 能否在不打破种植规则的情况下种入 n 朵花？能则返回True，不能则返回False。

// answers
impl Solution {
    pub fn p605_can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // codes
        let mut count = 0;
        let mut is_continuous = true;
        // 在前方加[1, 0]哨兵
        let mut zero_count = 1;

        // 0 0 1 => 1
        // 1 0 0 1 => 0
        // 1 0 0 0 1 => 1
        // 1 0 0 0 0 1 => 1 => (n + 1) / 2 - 1
        // 1 0 0 => 1 => n / 2
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                if is_continuous {
                    zero_count += 1;
                } else {
                    is_continuous = true;
                    zero_count = 1;
                }
            } else if is_continuous {
                count += (zero_count + 1) / 2 - 1;
                is_continuous = false;
                zero_count = 0;
            }
        }

        if is_continuous {
            count += zero_count / 2;
        }

        count >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p605_can_place_flowers_t1() {
        assert_eq!(
            Solution::p605_can_place_flowers(vec![1, 0, 0, 0, 1], 1),
            true
        );
    }

    #[test]
    fn p605_can_place_flowers_t2() {
        assert_eq!(
            Solution::p605_can_place_flowers(vec![1, 0, 0, 0, 1], 2),
            false
        );
    }

    #[test]
    fn p605_can_place_flowers_t3() {
        assert_eq!(Solution::p605_can_place_flowers(vec![0, 0, 0, 1], 2), false);
    }
}
