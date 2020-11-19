#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    fn inorder(root: Option<&Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(root) = root {
            Self::inorder(root.borrow().left.as_ref(), nums);
            nums.push(root.borrow().val);
            Self::inorder(root.borrow().right.as_ref(), nums);
        }
    }

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

    pub fn p1382_balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        let mut nums = vec![];
        Self::inorder(root.as_ref(), &mut nums);
        Self::helper(&nums, 0, nums.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree_ignore_parent, NULL};

    #[test]
    fn p1382_balance_bst_t1() {
        assert_eq!(
            Solution::p1382_balance_bst(build_tree_ignore_parent(&vec![
                1, NULL, 2, NULL, 3, NULL, 4
            ])),
            build_tree_ignore_parent(&vec![3, 2, 4, 1])
        );
    }
}
