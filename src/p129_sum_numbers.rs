#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// Given a binary tree containing digits from 0-9 only, each root-to-leaf path could represent a number.
// An example is the root-to-leaf path 1->2->3 which represents the number 123.
// Find the total sum of all root-to-leaf numbers.
// Note: A leaf is a node with no children.

// Input: [1,2,3]
//     1
//    / \
//   2   3
// Output: 25
// Explanation:
// The root-to-leaf path 1->2 represents the number 12.
// The root-to-leaf path 1->3 represents the number 13.
// Therefore, sum = 12 + 13 = 25.

// answers
// dfs
impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, pre: i32, nums: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            let curr = pre * 10 + node.val;
            if node.left.is_none() && node.right.is_none() {
                nums.push(curr);
            } else {
                Self::dfs(node.left.as_ref(), curr, nums);
                Self::dfs(node.right.as_ref(), curr, nums);
            }
        }
    }

    pub fn p129_sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // codes
        let mut nums: Vec<i32> = vec![];
        Self::dfs(root.as_ref(), 0, &mut nums);
        nums.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p129_sum_numbers_t1() {
        assert_eq!(
            Solution::p129_sum_numbers(btree![1, 2, 3]),
            25
        );
    }

    #[test]
    fn p129_sum_numbers_t2() {
        assert_eq!(
            Solution::p129_sum_numbers(btree![4, 9, 0, 5, 1]),
            1026
        );
    }
}
