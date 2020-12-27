#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 用一个大小为 m x n 的二维网格 grid 表示一个箱子。你有 n 颗球。箱子的顶部和底部都是开着的。
// 箱子中的每个单元格都有一个对角线挡板，跨过单元格的两个角，可以将球导向左侧或者右侧。
// 将球导向右侧的挡板跨过左上角和右下角，在网格中用 1 表示。
// 将球导向左侧的挡板跨过右上角和左下角，在网格中用 -1 表示。
// 在箱子每一列的顶端各放一颗球。每颗球都可能卡在箱子里或从底部掉出来。
// 如果球恰好卡在两块挡板之间的 "V" 形图案，或者被一块挡导向到箱子的任意一侧边上，就会卡住。
// 返回一个大小为 n 的数组 answer ，其中 answer[i] 是球放在顶部的第 i 列后从底部掉出来的那一列对应的下标，如果球卡在盒子里，则返回 -1 。

// answers
impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, row: usize, col: usize, m: usize, n: usize) -> i32 {
        // 退出条件，到达最后一层
        if m == row {
            return n as i32;
        }
        // 1. 碰壁
        if grid[m][n] == 1 && n + 1 == col || grid[m][n] == -1 && n == 0 {
            return -1;
        }
        // 2. 到下一层，这里优化了一下
        // println!("{},{},{}", m, n, grid[m][n]);
        if grid[m][n] == grid[m][(n as i32 + grid[m][n]) as usize] {
            return Self::dfs(&grid, row, col, m + 1, (n as i32 + grid[m][n]) as usize);
        }

        return -1;
    }

    pub fn p5210_find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // codes
        let mut ans = vec![];
        let row = grid.len();
        let col = grid[0].len();

        for i in 0..col {
            ans.push(Self::dfs(&grid, row, col, 0, i));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5210_find_ball_t1() {
        assert_eq!(
            Solution::p5210_find_ball(vec![
                vec![1, 1, 1, -1, -1],
                vec![1, 1, 1, -1, -1],
                vec![1, -1, -1, -1, -1],
                vec![-1, -1, -1, 1, 1],
                vec![1, 1, 1, 1, -1],
                vec![-1, -1, -1, -1, -1]
            ]),
            vec![0, -1, -1, -1, -1]
        );
    }

    #[test]
    fn p5210_find_ball_t2() {
        assert_eq!(Solution::p5210_find_ball(vec![vec![-1]]), vec![-1]);
    }
}
