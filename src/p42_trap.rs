#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p42_trap(height: Vec<i32>) -> i32 {
        // codes
        let mut stack: Vec<usize> = vec![];
        let mut ans: i32 = 0;

        // 单调递减栈
        // 记录数组下标是递增的，但对应的值是递减的
        for i in 0..height.len() {
            while !stack.is_empty() && height[i] > height[*stack.last().unwrap()] {
                let top = stack.pop().unwrap();
                if stack.is_empty() {
                    break;
                }
                let distance = (i - stack.last().unwrap() - 1) as i32;
                let bounded_height = height[i].min(height[*stack.last().unwrap()]) - height[top];
                ans += distance * bounded_height;
            }
            stack.push(i);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p42_trap_t1() {
        assert_eq!(
            Solution::p42_trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
    }

    #[test]
    fn p42_trap_t2() {
        assert_eq!(Solution::p42_trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
