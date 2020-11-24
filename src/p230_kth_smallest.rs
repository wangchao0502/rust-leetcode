#![allow(dead_code)]

// use mods
use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description
// Given a binary search tree, write a function kthSmallest to find the kth smallest element in it.

// stack = []
// while True:
//     while root:
//         stack.append(root)
//         root = root.left
//     root = stack.pop()
//     k -= 1
//     if not k:
//         return root.val
//     root = root.right

// answers
impl Solution {
    pub fn p230_kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // codes
        let mut stack = vec![];
        let mut k = k;
        let mut cur: Option<Rc<RefCell<TreeNode>>> = root;

        loop {
            while cur.is_some() {
                stack.push(cur.clone());
                cur = cur.unwrap().borrow().left.clone();
            }

            cur = stack.pop().unwrap();
            k -= 1;

            if k == 0 {
                return cur.unwrap().borrow().val;
            }

            cur = cur.unwrap().borrow().right.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree_node::{build_tree, NULL};

    #[test]
    fn p230_kth_smallest_t1() {
        assert_eq!(
            Solution::p230_kth_smallest(build_tree(&vec![3, 1, 4, NULL, 2]), 1),
            1
        );
    }
    #[test]
    fn p230_kth_smallest_t2() {
        assert_eq!(
            Solution::p230_kth_smallest(build_tree(&vec![5, 3, 6, 2, 4, NULL, NULL, 1]), 3),
            3
        );
    }
}
