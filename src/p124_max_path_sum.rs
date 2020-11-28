#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given a non-empty binary tree, find the maximum path sum.
// For this problem, a path is defined as any node sequence from
// some starting node to any node in the tree along the parent-child connections.
// The path must contain at least one node and does not need to go through the root.

// answers
impl Solution {
    fn max_gain(root: Option<&Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(root) = root {
            // 递归计算左右子节点的最大贡献值
            // 只有在最大贡献值大于 0 时，才会选取对应子节点
            let left_gain = 0.max(Self::max_gain(root.borrow().left.as_ref(), max_sum));
            let right_gain = 0.max(Self::max_gain(root.borrow().right.as_ref(), max_sum));

            // 节点的最大路径和取决于该节点的值与该节点的左右子节点的最大贡献值
            let price_new_path = root.borrow().val + left_gain + right_gain;

            // 更新答案
            *max_sum = price_new_path.max(*max_sum);

            // 返回节点的最大贡献值
            root.borrow().val + left_gain.max(right_gain)
        } else {
            0
        }
    }

    pub fn p124_max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // codes
        let mut ans = i32::MIN;
        Self::max_gain(root.as_ref(), &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p124_max_path_sum_t1() {
        assert_eq!(Solution::p124_max_path_sum(btree![1, 2, 3]), 6);
    }

    #[test]
    fn p124_max_path_sum_t2() {
        assert_eq!(
            Solution::p124_max_path_sum(btree![-10, 9, 20, null, null, 15, 7]),
            42
        );
    }
}
