#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
// quick sort
impl Solution {
    pub fn p973_k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // codes
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for v in points {
            let distance = v[0].pow(2) + v[1].pow(2);
            if heap.len() < k as usize {
                heap.push((distance, v));
            } else {
                if distance < heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push((distance, v));
                }
            }
        }
        heap.into_vec().into_iter().map(|t| t.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p973_k_closest_t1() {
        assert_eq!(
            Solution::p973_k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![-2, 4], vec![3, 3]]
        );
    }
}
