#![allow(dead_code)]

// use mods
use crate::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// answers
// InOrder Binary Search Tree
impl Solution {
    fn go(root: &Option<Rc<RefCell<TreeNode>>>, pre: i32, ans: i32) -> (i32, i32) {
        match root {
            Some(node) => {
                let b = node.borrow();
                let (mut pre, mut ans) = Self::go(&b.left, pre, ans);
                ans = ans.min(b.val.saturating_sub(pre));
                pre = b.val;
                Self::go(&b.right, pre, ans)
            }
            _ => (pre, ans),
        }
    }

    pub fn p530_get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // codes
        Self::go(&root, i32::MIN, i32::MAX).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn p530_get_minimum_difference_t1() {
        assert_eq!(
            Solution::p530_get_minimum_difference(build_tree(&vec![9, 7, 15, 3, NULL, NULL, 20])),
            2
        );
        assert_eq!(
            Solution::p530_get_minimum_difference(build_tree(&vec![2, 1, 3])),
            1
        );
        assert_eq!(
            Solution::p530_get_minimum_difference(build_tree(&vec![
                4, 3, 5, 2, NULL, NULL, 6, 1, NULL, NULL, NULL, NULL, NULL, NULL, 7
            ])),
            1
        );
    }
}
