#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
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
    use leetcode_prelude::btree;

    #[test]
    fn p897_increasing_bst_t1() {
        assert_eq!(
            Solution::p897_increasing_bst(btree![
                5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9
            ]),
            btree![1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9]
        );
    }
}
