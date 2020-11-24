#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
// According to the definition of LCA on Wikipedia:
// “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that
// has both p and q as descendants (where we allow a node to be a descendant of itself).”

// answers
impl Solution {
    fn find(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if node.borrow().val == p || node.borrow().val == q {
                    return root.clone();
                }
                let left = Self::find(&node.borrow().left, p, q);
                let right = Self::find(&node.borrow().right, p, q);
                if left == None {
                    right
                } else if right == None {
                    left
                } else {
                    root.clone()
                }
            }
            None => None,
        }
    }

    pub fn p236_lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        Self::find(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p236_lowest_common_ancestor_t1() {
        assert_eq!(
            Solution::p236_lowest_common_ancestor(
                btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
                btree![5],
                btree![1]
            ),
            btree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
        );
    }
}
