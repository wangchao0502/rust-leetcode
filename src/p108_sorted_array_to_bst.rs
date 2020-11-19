#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
// For this problem, a height-balanced binary tree is defined as a binary tree
// in which the depth of the two subtrees of every node never differ by more than 1.

// answers
impl Solution {
    fn helper(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            None
        } else {
            let mid = (start + end) / 2;
            let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            node.borrow_mut().left = Self::helper(nums, start, mid);
            node.borrow_mut().right = Self::helper(nums, mid + 1, end);
            Some(node)
        }
    }

    pub fn p108_sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        Self::helper(&nums, 0, nums.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree_ignore_parent, NULL};

    #[test]
    fn p108_sorted_array_to_bst_t1() {
        assert_eq!(
            Solution::p108_sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            build_tree_ignore_parent(&vec![0, -3, 9, -10, NULL, 5])
        );
    }
}
