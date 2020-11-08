#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p226_invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        if let Some(r) = root.as_ref() {
            let left = r.borrow().left.clone();
            let right = r.borrow().right.clone();
            r.borrow_mut().left = Solution::p226_invert_tree(right);
            r.borrow_mut().right = Solution::p226_invert_tree(left);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::build_tree;

    #[test]
    fn p226_invert_tree_t1() {
        assert_eq!(
            Solution::p226_invert_tree(build_tree(&vec![4, 2, 7, 1, 3, 6, 9])),
            build_tree(&vec![4, 7, 2, 9, 6, 3, 1])
        );
    }
}
