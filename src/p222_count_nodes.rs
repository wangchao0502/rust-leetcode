#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given a complete binary tree, count the number of nodes.

// answers
impl Solution {
    fn count_levels(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level = 0;
        let mut cur = node;

        while let Some(tree_node) = cur {
            level += 1;
            cur = tree_node.borrow().left.clone();
        }
        level
    }

    pub fn p222_count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // codes
        if root.is_none() {
            return 0;
        }

        let tree_node = root.as_ref().unwrap().borrow();
        let left = Self::count_levels(tree_node.left.clone());
        let right = Self::count_levels(tree_node.right.clone());

        return if left == right {
            Solution::p222_count_nodes(tree_node.right.clone()) + (1 << left)
        } else {
            Solution::p222_count_nodes(tree_node.left.clone()) + (1 << right)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p222_count_nodes_t1() {
        assert_eq!(Solution::p222_count_nodes(btree![1, 2, 3, 4, 5, 6]), 6);
    }
}
