#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given a characters array tasks, representing the tasks a CPU needs to do,
// where each letter represents a different task. Tasks could be done in any order.
// Each task is done in one unit of time.
// For each unit of time, the CPU could complete either one task or just be idle.
// However, there is a non-negative integer n that represents the cooldown period
// between two same tasks (the same letter in the array), that is that there must be
// at least n units of time between any two same tasks.
// Return the least number of units of times that the CPU will take to finish all the given tasks.

// answers
// https://leetcode-cn.com/problems/task-scheduler/solution/tong-zi-by-popopop/
impl Solution {
    pub fn p621_least_interval(tasks: Vec<char>, n: i32) -> i32 {
        // codes
        let len = tasks.len();
        let mut count = vec![0; 26];

        for c in tasks {
            count[(c as u8 - b'A') as usize] += 1;
        }

        count.sort();
        count.reverse();

        let mut cnt = 1;
        while cnt < count.len() && count[cnt] == count[0] {
            cnt += 1;
        }

        (len as i32).max(cnt as i32 + (n + 1) * (count[0] - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p621_least_interval_t1() {
        assert_eq!(
            Solution::p621_least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            ),
            16
        );
    }

    #[test]
    fn p621_least_interval_t2() {
        assert_eq!(
            Solution::p621_least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
            6
        );
    }

    #[test]
    fn p621_least_interval_t3() {
        assert_eq!(
            Solution::p621_least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }
}
