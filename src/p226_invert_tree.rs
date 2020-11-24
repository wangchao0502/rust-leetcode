#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// add structs

// answers
impl Solution {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let left = Self::helper(root.borrow().left.as_ref());
            let right = Self::helper(root.borrow().right.as_ref());
            root.borrow_mut().left = right;
            root.borrow_mut().right = left;
            return Some(Rc::clone(root));
        }
        None
    }

    pub fn p226_invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // clone tree node
        // if let Some(r) = root.as_ref() {
        //     let left = r.borrow().left.clone();
        //     let right = r.borrow().right.clone();
        //     r.borrow_mut().left = Solution::p226_invert_tree(right);
        //     r.borrow_mut().right = Solution::p226_invert_tree(left);
        // }
        // root
        Self::helper(root.as_ref());
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p226_invert_tree_t1() {
        assert_eq!(
            Solution::p226_invert_tree(btree![4, 2, 7, 1, 3, 6, 9]),
            btree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}
