#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// Given two binary trees and imagine that when you put one of them to cover the other,
// some nodes of the two trees are overlapped while the others are not.
// You need to merge them into a new binary tree. The merge rule is that if two nodes overlap,
// then sum node values up as the new value of the merged node.
// Otherwise, the NOT null node will be used as the node of new tree.

// answers
impl Solution {
    pub fn p617_merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        match (t1.clone(), t2) {
            (Some(p), Some(q)) => {
                let (mut p, q) = (p.borrow_mut(), q.borrow());
                p.val += q.val;
                p.left = Self::p617_merge_trees(p.left.clone(), q.left.clone());
                p.right = Self::p617_merge_trees(p.right.clone(), q.right.clone());
                t1
            }
            (t, None) | (None, t) => t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree, NULL};

    #[test]
    fn p617_merge_trees_t1() {
        assert_eq!(
            Solution::p617_merge_trees(
                build_tree(&vec![1, 3, 2, 5]),
                build_tree(&vec![2, 1, 3, NULL, 4, NULL, 7])
            ),
            build_tree(&vec![3, 4, 5, 5, 4, NULL, 7])
        );
    }
}
