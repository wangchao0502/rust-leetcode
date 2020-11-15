#![allow(dead_code)]

// use mods

pub struct Solution {}

// Given two arrays arr1 and arr2, the elements of arr2 are distinct,
// and all elements in arr2 are also in arr1.

// Sort the elements of arr1 such that the relative ordering of
// items in arr1 are the same as in arr2.
// Elements that don't appear in arr2 should be placed
// at the end of arr1 in ascending order.

// answers
impl Solution {
    pub fn p1122_relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        // codes
        use std::collections::{HashMap, HashSet};
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let mut ans = vec![];

        for i in arr1 {
            map.entry(i).and_modify(|x| *x += 1).or_insert(1);
            set.insert(i);
        }
        for i in arr2 {
            for _ in 0..*map.get(&i).unwrap() {
                ans.push(i);
            }
            set.remove(&i);
        }

        // attach extra numer in ascending order
        let mut rest = vec![];

        for (k, v) in map {
            if set.contains(&k) {
                rest = [rest, vec![k; v]].concat();
            }
        }

        rest.sort();
        [ans, rest].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1122_relative_sort_array_t1() {
        assert_eq!(
            Solution::p1122_relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }

    #[test]
    fn p1122_relative_sort_array_t2() {
        assert_eq!(Solution::p1122_relative_sort_array(vec![], vec![]), vec![]);
    }

    #[test]
    fn p1122_relative_sort_array_t3() {
        assert_eq!(
            Solution::p1122_relative_sort_array(vec![1], vec![]),
            vec![1]
        );
    }
}
