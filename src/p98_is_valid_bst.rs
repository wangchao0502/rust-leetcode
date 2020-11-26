#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, low: f64, high: f64) -> bool {
        match root {
            Some(root) => {
                let val = root.borrow().val as f64;
                if val <= low || val >= high {
                    return false;
                } else {
                    Self::helper(root.borrow().left.as_ref(), low, val)
                        && Self::helper(root.borrow().right.as_ref(), val, high)
                }
            }
            None => true,
        }
    }

    pub fn p98_is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // codes
        Self::helper(root.as_ref(), f64::MIN, f64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p98_is_valid_bst_t1() {
        assert_eq!(
            Solution::p98_is_valid_bst(btree![5, 1, 4, null, null, 3, 6]),
            false
        );
    }

    #[test]
    fn p98_is_valid_bst_t2() {
        assert_eq!(Solution::p98_is_valid_bst(btree![2, 1, 3]), true);
    }

    #[test]
    fn p98_is_valid_bst_t3() {
        assert_eq!(Solution::p98_is_valid_bst(btree![1, 1]), false);
    }
}
