#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p1030_all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        // codes
        let mut point: Vec<Vec<i32>> = vec![];

        for i in 0..r {
            for j in 0..c {
                point.push(vec![i, j]);
            }
        }

        point.sort_unstable_by_key(|v| (v[0] - r0).abs() + (v[1] - c0).abs());
        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1030_all_cells_dist_order_t1() {
        assert_eq!(
            Solution::p1030_all_cells_dist_order(1, 2, 0, 0),
            vec![vec![0, 0], vec![0, 1]]
        );
    }
}
