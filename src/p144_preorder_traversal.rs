#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// add structs

// answers
// preorder traverse, dfs
impl Solution {
    fn preorder(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            ans.push(node.val);
            Self::preorder(&node.left, ans);
            Self::preorder(&node.right, ans);
        }
    }

    pub fn p144_preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // codes
        let mut ans = vec![];
        Self::preorder(&root, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p144_preorder_traversal_t1() {
        assert_eq!(
            Solution::p144_preorder_traversal(btree![1, null, 2, 3]),
            vec![1, 2, 3]
        );
    }
}
