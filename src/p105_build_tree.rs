#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given preorder and inorder traversal of a tree, construct the binary tree.

// 对于任意一颗树而言，前序遍历的形式总是
// [ 根节点, [左子树的前序遍历结果], [右子树的前序遍历结果] ]

// 即根节点总是前序遍历中的第一个节点。而中序遍历的形式总是
// [ [左子树的中序遍历结果], 根节点, [右子树的中序遍历结果] ]

// 前序遍历确定根结点，中序遍历确定左右子树

// answers
impl Solution {
    fn helper(
        preorder: &Vec<i32>,
        po: (usize, usize),
        inorder: &Vec<i32>,
        io: (usize, usize),
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if po.0 >= po.1 {
            return None;
        }
        if io.0 >= io.1 {
            return None;
        }
        let mut node = TreeNode::new(preorder[po.0]);
        let len = match inorder
            .iter()
            .skip(io.0)
            .take(io.1 - io.0)
            .position(|&x| preorder[po.0] == x)
        {
            Some(idx) => idx,
            None => io.1 - io.0,
        };

        node.left = Self::helper(
            preorder,
            (po.0 + 1, po.0 + 1 + len),
            inorder,
            (io.0, io.0 + len),
        );
        node.right = Self::helper(
            preorder,
            (po.0 + len + 1, po.1),
            inorder,
            (io.0 + len + 1, io.1),
        );
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn p105_build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // solution 1
        Self::helper(&preorder, (0, preorder.len()), &inorder, (0, inorder.len()))

        // solution 2, use slice
        // fn bt(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        //     if preorder.len() == 0 || inorder.len() == 0 {
        //         return None;
        //     }
        //     let root = Rc::new(RefCell::new(TreeNode {
        //         val: preorder[0],
        //         left: None,
        //         right: None,
        //     }));
        //     let i = inorder.iter().position(|&v| v == preorder[0]).unwrap();
        //     root.borrow_mut().left = bt(&preorder[1..1 + i], &inorder[..i]);
        //     root.borrow_mut().right = bt(&preorder[1 + i..], &inorder[i + 1..]);
        //     Some(root)
        // }
        // bt(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p105_build_tree_t1() {
        assert_eq!(
            Solution::p105_build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            btree![3, 9, 20, null, null, 15, 7]
        );
    }
}
