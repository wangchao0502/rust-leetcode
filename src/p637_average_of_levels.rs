#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given a non-empty binary tree, return the average value of
// the nodes on each level in the form of an array.

// answers
impl Solution {
    pub fn p637_average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        // codes
        let mut res: Vec<f64> = vec![];
        let mut queue = vec![];

        if let Some(root) = root {
            queue.push(root)
        }

        while !queue.is_empty() {
            let mut sum = 0.0;
            let mut count = 0;
            let mut temp = vec![];

            while let Some(node) = queue.pop() {
                let mut node = node.borrow_mut();
                sum += node.val as f64;
                count += 1;
                if let Some(left) = node.left.take() {
                    temp.push(left);
                }
                if let Some(right) = node.right.take() {
                    temp.push(right);
                }
            }

            res.push(sum / count as f64);
            queue = temp;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree, NULL};

    #[test]
    fn p637_average_of_levels_t1() {
        assert_eq!(
            Solution::p637_average_of_levels(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            vec![3.0, 14.5, 11.0]
        );
    }
}
