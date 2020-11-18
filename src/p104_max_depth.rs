#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    // fn depth(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    //     if let Some(r) = root {
    //         1 + std::cmp::max(
    //             Self::depth(r.borrow().left.as_ref()),
    //             Self::depth(r.borrow().right.as_ref()),
    //         )
    //     } else {
    //         0
    //     }
    // }

    // pub fn p104_max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if root.is_some() {
    //         Self::depth(root.as_ref())
    //     } else {
    //         0
    //     }
    // }

    // 迭代法
    pub fn p104_max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        // record root and depth
        let mut stk = vec![(root.clone(), 0)];
        while !stk.is_empty() {
            let (maybe, depth) = stk.pop().unwrap();
            ans = ans.max(depth);
            if let Some(node) = maybe {
                let node = node.borrow();
                stk.push((node.left.clone(), depth + 1));
                stk.push((node.right.clone(), depth + 1));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree_ignore_parent, NULL};

    #[test]
    fn p104_max_depth_t1() {
        assert_eq!(
            Solution::p104_max_depth(build_tree_ignore_parent(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            3
        );
    }
}
