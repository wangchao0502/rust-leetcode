#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 我们称一个分割整数数组的方案是 好的 ，当它满足：
// 数组被分成三个 非空 连续子数组，从左至右分别命名为 left ， mid ， right 。
// left 中元素和小于等于 mid 中元素和，mid 中元素和小于等于 right 中元素和。
// 给你一个 非负 整数数组 nums ，请你返回 好的 分割 nums 方案数目。由于答案可能会很大，请你将结果对 109 + 7 取余后返回。

// answers
impl Solution {
    pub fn p5643_ways_to_split(nums: Vec<i32>) -> i32 {
        // codes
        // two pointers
        let len = nums.len();
        let mut s = vec![0; len + 1];

        // 前缀和
        for i in 1..=len {
            s[i] = s[i - 1] + nums[i - 1];
        }

        let mut ans: i64 = 0;
        let mut m = 2;

        for i in 1..=len - 2 {
            let left_sum = s[i];
            m = m.max(i + 1);
            while m < len && s[m] - left_sum < left_sum {
                m += 1;
            }
            if m == len {
                break;
            }
            let mut lo = m;
            let mut hi = len - 1;
            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);
                if s[len] - s[mid] < s[mid] - left_sum {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }
            ans += (hi + 1 - m) as i64;
        }

        (ans % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5643_ways_to_split_t1() {
        assert_eq!(Solution::p5643_ways_to_split(vec![1, 2, 2, 2, 5, 0]), 3);
    }

    #[test]
    fn p5643_ways_to_split_t2() {
        assert_eq!(Solution::p5643_ways_to_split(vec![3, 2, 1]), 0);
    }
}
