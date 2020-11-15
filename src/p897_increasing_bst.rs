#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// Given a binary search tree, rearrange the tree in in-order
// so that the leftmost node in the tree is now the root of the tree,
// and every node has no left child and only 1 right child.

// answers
impl Solution {
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(root) = root {
            Self::inorder(root.borrow().left.clone(), nums);
            nums.push(root.borrow().val);
            Self::inorder(root.borrow().right.clone(), nums);
        }
    }
    pub fn p897_increasing_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        let mut nums: Vec<i32> = vec![];
        Self::inorder(root, &mut nums);

        let mut curr = Some(Rc::new(RefCell::new(TreeNode::new(0)))).clone();
        let mut head = curr.clone();

        // build tree
        for i in nums {
            curr.as_ref().unwrap().borrow_mut().right =
                Some(Rc::new(RefCell::new(TreeNode::new(i))));
            curr = curr.clone().unwrap().borrow().right.clone();
        }
        head = head.clone().unwrap().borrow().right.clone();
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree_ignore_parent, NULL};

    #[test]
    fn p897_increasing_bst_t1() {
        assert_eq!(
            Solution::p897_increasing_bst(build_tree_ignore_parent(&vec![
                5, 3, 6, 2, 4, NULL, 8, 1, NULL, NULL, NULL, 7, 9
            ])),
            build_tree_ignore_parent(&vec![
                1, NULL, 2, NULL, 3, NULL, 4, NULL, 5, NULL, 6, NULL, 7, NULL, 8, NULL, 9
            ])
        );
    }
}
