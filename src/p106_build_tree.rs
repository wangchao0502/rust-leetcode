#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// 根据一棵树的中序遍历与后序遍历构造二叉树。
// 注意: 你可以假设树中没有重复的元素。

// answers
impl Solution {
    fn build_sub_tree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }

        let len = inorder.len();
        let mut node = TreeNode::new(postorder[len - 1]);
        if inorder.len() == 1 {
            return Some(Rc::new(RefCell::new(node)));
        }

        let mut idx = 0;
        let val = postorder[len - 1];
        // find left and right
        for i in 0..len {
            if inorder[i] == val {
                idx = i;
                break;
            }
        }

        // divide left tree and right tree
        node.left = Self::build_sub_tree(&inorder[0..idx], &postorder[0..idx]);
        node.right = Self::build_sub_tree(&inorder[idx + 1..], &postorder[idx..len - 1]);

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn p106_build_tree(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        Self::build_sub_tree(&inorder[..], &postorder[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p106_build_tree_t1() {
        assert_eq!(
            Solution::p106_build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            btree![3, 9, 20, null, null, 15, 7]
        );
    }
}
