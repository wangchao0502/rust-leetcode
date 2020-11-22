#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p94_inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut stack = Vec::new();
        let mut r = root.clone();
        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();
            if let Some(node) = r {
                res.push(node.borrow().val);
                r = node.borrow().right.clone();
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
    fn p94_inorder_traversal_t1() {
        assert_eq!(
            Solution::p94_inorder_traversal(build_tree_ignore_parent(&vec![1, NULL, 2, 3])),
            vec![1, 3, 2]
        );
    }
}
