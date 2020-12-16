#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 平面上有 n 个点，点的位置用整数坐标表示 points[i] = [xi, yi]。请你计算访问所有这些点需要的最小时间（以秒为单位）。
// 你可以按照下面的规则在平面上移动：
// 每一秒沿水平或者竖直方向移动一个单位长度，或者跨过对角线（可以看作在一秒内向水平和竖直方向各移动一个单位长度）。
// 必须按照数组中出现的顺序来访问这些点。

// answers
impl Solution {
    pub fn p1266_min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        // codes
        let mut ans = 0;

        for i in 1..points.len() {
            let delta_x = (points[i][0] - points[i - 1][0]).abs();
            let delta_y = (points[i][1] - points[i - 1][1]).abs();

            ans += delta_x.max(delta_y);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1266_min_time_to_visit_all_points_t1() {
        assert_eq!(
            Solution::p1266_min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
    }

    #[test]
    fn p1266_min_time_to_visit_all_points_t2() {
        assert_eq!(
            Solution::p1266_min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
