#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    // fn dfs(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, sum: &mut i32) {
    //     if let Some(root) = root {
    //         let node = root.borrow();
    //         if node.val >= low && node.val <= high {
    //             *sum += node.val;
    //             Self::dfs(node.left.clone(), low, high, sum);
    //             Self::dfs(node.right.clone(), low, high, sum);
    //         }
    //         if node.val < low {
    //             Self::dfs(node.right.clone(), low, high, sum);
    //         }
    //         if node.val > high {
    //             Self::dfs(node.left.clone(), low, high, sum);
    //         }
    //     }
    // }

    // pub fn p938_range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    //     let mut sum = 0;
    //     Self::dfs(root, low, high, &mut sum);
    //     sum
    // }

    // 深度优先，迭代解法
    // 没有消耗栈空间
    pub fn p938_range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut res = 0;
        let mut stack = vec![root];
        while let Some(peek) = stack.pop() {
            if let Some(peek) = peek {
                let peek = peek.borrow();
                if low <= peek.val && peek.val <= high {
                    res += peek.val;
                }
                stack.push(peek.right.clone());
                stack.push(peek.left.clone());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree_ignore_parent, NULL};

    #[test]
    fn p938_range_sum_bst_t1() {
        assert_eq!(
            Solution::p938_range_sum_bst(
                build_tree_ignore_parent(&vec![10, 5, 15, 3, 7, NULL, 18]),
                7,
                15
            ),
            32
        );
    }
}
