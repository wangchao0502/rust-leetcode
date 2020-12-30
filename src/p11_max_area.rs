#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。
// 在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0) 。
// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
// 说明：你不能倾斜容器。

// answers
impl Solution {
    pub fn p11_max_area(height: Vec<i32>) -> i32 {
        // codes
        let (mut left, mut right, mut ans) = (0 as usize, height.len() - 1, -1);
        while left < right {
            ans = ans.max(height[left].min(height[right]) * (right as i32 - left as i32));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p11_max_area_t1() {
        assert_eq!(Solution::p11_max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
