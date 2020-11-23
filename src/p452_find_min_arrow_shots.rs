#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// There are some spherical balloons spread in two-dimensional space. For each balloon,
// provided input is the start and end coordinates of the horizontal diameter.
// Since it's horizontal, y-coordinates don't matter, and hence the x-coordinates of start and end of the diameter suffice.
// The start is always smaller than the end.
// An arrow can be shot up exactly vertically from different points along the x-axis.
// A balloon with xstart and xend bursts by an arrow shot at x if xstart ≤ x ≤ xend.
// There is no limit to the number of arrows that can be shot. An arrow once shot keeps traveling up infinitely.
// Given an array points where points[i] = [xstart, xend], return the minimum number of arrows that must be shot to burst all balloons.

// answers
impl Solution {
    pub fn p452_find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        // codes
        let mut ans = 0;
        let mut i = 0;

        points.sort_unstable_by_key(|x| x[1]);

        while i < points.len() {
            let right = points[i][1];
            i += 1;
            while i < points.len() && points[i][0] <= right {
                i += 1;
            }
            ans += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p452_find_min_arrow_shots_t1() {
        assert_eq!(
            Solution::p452_find_min_arrow_shots(vec![
                vec![10, 16],
                vec![2, 8],
                vec![1, 6],
                vec![7, 12]
            ]),
            2
        );
    }
}
