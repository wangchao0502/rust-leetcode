#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

// answers
impl Solution {
    pub fn p101_is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // codes
        fn check(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(l), Some(r)) => {
                    let (l, r) = (l.borrow(), r.borrow());
                    l.val == r.val && check(&l.left, &r.right) && check(&l.right, &r.left)
                }
                _ => false,
            }
        }
        check(&root, &root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p101_is_symmetric_t1() {
        assert_eq!(
            Solution::p101_is_symmetric(btree![1, 2, 2, null, 3, null, 3]),
            false
        );
    }
}
