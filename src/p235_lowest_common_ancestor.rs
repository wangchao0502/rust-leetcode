#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// 给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。
// 百度百科中最近公共祖先的定义为：“对于有根树 T 的两个结点 p、q，最近公共祖先表示为一个结点 x，
// 满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”

// answers
// 1. 二次遍历，找到两次遍历路径的相同节点
// 2. 一次遍历，利用搜索二叉树的特性，比当前值都大或都小，在一侧；一个大一个小，为当前节点；如果等于，当前节点。
impl Solution {
    fn collect_ancestor(
        root: Option<&Rc<RefCell<TreeNode>>>,
        p_val: i32,
        q_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root.unwrap().borrow();

        // 同大同小
        if (node.val - p_val) * (node.val - q_val) <= 0 {
            return Some(Rc::clone(root.unwrap()));
        }
        if node.val > p_val {
            Self::collect_ancestor(node.left.as_ref(), p_val, q_val)
        } else {
            Self::collect_ancestor(node.right.as_ref(), p_val, q_val)
        }
    }

    pub fn p235_lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // codes
        Self::collect_ancestor(
            root.as_ref(),
            p.unwrap().borrow().val,
            q.unwrap().borrow().val,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p235_lowest_common_ancestor_t1() {
        assert_eq!(
            Solution::p235_lowest_common_ancestor(
                btree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
                btree![2],
                btree![8]
            ),
            btree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]
        );
    }
}
