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
    fn inorder(root: Option<&Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(root) = root {
            Self::inorder(root.borrow().left.as_ref(), arr);
            arr.push(root.borrow().val);
            Self::inorder(root.borrow().right.as_ref(), arr);
        }
    }
    pub fn p897_increasing_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        let mut arr = vec![];
        Self::inorder(root.as_ref(), &mut arr);

        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = dummy.clone();
        let mut head = None;

        // build tree
        arr.into_iter().for_each(|val| {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            cur.borrow_mut().right = Some(node.clone());
            head.get_or_insert(node.clone());
            cur = node;
        });

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
