#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
// dfs
impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, length: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let left_len = Self::dfs(root.unwrap().borrow().left.as_ref(), length);
        let right_len = Self::dfs(root.unwrap().borrow().right.as_ref(), length);
        let cur_len = left_len + right_len + 1;

        if cur_len > *length {
            *length = cur_len;
        }

        left_len.max(right_len) + 1
    }

    pub fn p543_diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // codes
        if root.is_none() {
            return 0;
        }
        let mut length = 0;
        Self::dfs(root.as_ref(), &mut length);
        length - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p543_diameter_of_binary_tree_t1() {
        assert_eq!(
            Solution::p543_diameter_of_binary_tree(btree![1, 2, 3, 4, 5]),
            3
        );
    }
}
