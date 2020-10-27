#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// sort and get index
impl Solution {
    pub fn p1365_smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // codes
        let mut sorted = nums.clone();
        sorted.sort();

        use std::collections::HashMap;
        let mut map = HashMap::new();

        for (i, num) in sorted.iter().enumerate() {
            map.entry(num).or_insert(i);
        }

        nums.iter().map(|x| map[x] as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1365_smaller_numbers_than_current_t1() {
        assert_eq!(
            Solution::p1365_smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
}
