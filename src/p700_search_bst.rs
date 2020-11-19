#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given the root node of a binary search tree (BST) and a value.
// You need to find the node in the BST that the node's value equals the given value.
// Return the subtree rooted with that node.
// If such node doesn't exist, you should return NULL.

// answers
impl Solution {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            if root.borrow().val == val {
                Some(Rc::clone(root))
            } else if root.borrow().val > val {
                Self::helper(root.borrow().left.as_ref(), val)
            } else {
                Self::helper(root.borrow().right.as_ref(), val)
            }
        } else {
            None
        }
    }

    pub fn p700_search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        Self::helper(root.as_ref(), val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::build_tree_ignore_parent;

    #[test]
    fn p700_search_bst_t1() {
        assert_eq!(
            Solution::p700_search_bst(build_tree_ignore_parent(&vec![4, 2, 7, 1, 3]), 2),
            build_tree_ignore_parent(&vec![2, 1, 3])
        );
    }
}
