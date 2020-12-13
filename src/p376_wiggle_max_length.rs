#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 如果连续数字之间的差严格地在正数和负数之间交替，则数字序列称为摆动序列。
// 第一个差（如果存在的话）可能是正数或负数。少于两个元素的序列也是摆动序列。
// 例如， [1,7,4,9,2,5] 是一个摆动序列，因为差值 (6,-3,5,-7,3) 是正负交替出现的。
// 相反, [1,4,7,2,5] 和 [1,7,4,5,5] 不是摆动序列，第一个序列是因为它的前两个差值都是正数，第二个序列是因为它的最后一个差值为零。
// 给定一个整数序列，返回作为摆动序列的最长子序列的长度。
// 通过从原始序列中删除一些（也可以不删除）元素来获得子序列，剩下的元素保持其原始顺序。

// answers
impl Solution {
    pub fn p376_wiggle_max_length(nums: Vec<i32>) -> i32 {
        // codes
        // up[i] 表示 nums[0:i] 中最后两个数字递增的最长摆动序列长度
        // down[i] 表示 nums[0:i] 中最后两个数字递减的最长摆动序列长度，只有一个数字时默认为 1。
        let mut down = 1;
        let mut up = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                up = down + 1;
            } else if nums[i] < nums[i - 1] {
                down = up + 1;
            }
        }

        return if nums.len() == 0 { 0 } else { down.max(up) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p376_wiggle_max_length_t1() {
        assert_eq!(Solution::p376_wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    }

    #[test]
    fn p376_wiggle_max_length_t2() {
        assert_eq!(
            Solution::p376_wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
    }

    #[test]
    fn p376_wiggle_max_length_t3() {
        assert_eq!(
            Solution::p376_wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
    }
}
