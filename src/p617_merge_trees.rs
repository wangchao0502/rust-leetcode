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
    fn helper(
        t1: Option<&Rc<RefCell<TreeNode>>>,
        t2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (None, None) => None,
            (None, t) | (t, None) => Some(Rc::clone(t.unwrap())),
            (Some(t1), Some(t2)) => {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: t1.borrow().val + t2.borrow().val,
                    left: Self::helper(t1.borrow().left.as_ref(), t2.borrow().left.as_ref()),
                    right: Self::helper(t2.borrow().right.as_ref(), t1.borrow().right.as_ref()),
                })))
            }
        }
    }

    pub fn p617_merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(t1.as_ref(), t2.as_ref())
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
