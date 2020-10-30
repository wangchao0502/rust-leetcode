#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p463_island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        // codes
        let mut perimeter = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    perimeter += 4;
                    if i > 0 {
                        perimeter -= grid[i - 1][j] * 2;
                    }
                    if j > 0 {
                        perimeter -= grid[i][j - 1] * 2;
                    }
                }
            }
        }

        perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p463_island_perimeter_t1() {
        assert_eq!(
            Solution::p463_island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }

    #[test]
    fn p463_island_perimeter_t2() {
        assert_eq!(Solution::p463_island_perimeter(vec![vec![1]]), 4);
    }

    #[test]
    fn p463_island_perimeter_t3() {
        assert_eq!(Solution::p463_island_perimeter(vec![vec![1, 0]]), 4);
    }
}
