#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    fn compare(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(n1), Some(n2)) => {
                if n1.borrow().val == n2.borrow().val {
                    Self::compare(n1.borrow().left.as_ref(), n2.borrow().left.as_ref())
                        && Self::compare(n1.borrow().right.as_ref(), n2.borrow().right.as_ref())
                } else {
                    false
                }
            }
            (None, None) => true,
            (_, _) => false,
        }
    }

    pub fn p100_is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // codes
        Self::compare(p.as_ref(), q.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p100_is_same_tree_t1() {
        assert_eq!(
            Solution::p100_is_same_tree(btree![1, 2, 3], btree![1, 2, 3]),
            true
        );
    }

    #[test]
    fn p100_is_same_tree_t2() {
        assert_eq!(
            Solution::p100_is_same_tree(btree![1, 2], btree![1, null, 2]),
            false
        );
    }
}
