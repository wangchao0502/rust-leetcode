#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 请你将一些箱子装在 一辆卡车 上。给你一个二维数组 boxTypes ，其中 boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi] ：
// numberOfBoxesi 是类型 i 的箱子的数量。
// numberOfUnitsPerBoxi 是类型 i 每个箱子可以装载的单元数量。
// 整数 truckSize 表示卡车上可以装载 箱子 的 最大数量 。只要箱子数量不超过 truckSize ，你就可以选择任意箱子装到卡车上。
// 返回卡车可以装载 单元 的 最大 总数。

// answers
impl Solution {
    pub fn p5641_maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        // codes
        box_types.sort_by_key(|x| x[1]);
        box_types.reverse();

        let mut rest = truck_size;
        let mut ans: i32 = 0;
        let mut i: usize = 0;

        while rest > 0 && i < box_types.len() {
            if rest >= box_types[i][0] {
                rest -= box_types[i][0];
                ans += box_types[i][0] * box_types[i][1];
                i += 1;
            } else {
                ans += rest * box_types[i][1];
                break;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p5641_maximum_units_t1() {
        assert_eq!(
            Solution::p5641_maximum_units(
                vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]],
                10
            ),
            91
        );
    }
}
