#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
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
    use leetcode_prelude::btree;

    #[test]
    fn p1382_balance_bst_t1() {
        assert_eq!(
            Solution::p1382_balance_bst(btree![
                1, null, 2, null, 3, null, 4
            ]),
            btree![3, 2, 4, 1]
        );
    }
}
