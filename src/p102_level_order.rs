#![allow(dead_code)]

// use mods
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p102_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // codes
        let mut deque = match root {
            Some(r) => vec![r],
            None => vec![],
        };
        let mut ans = vec![];

        while !deque.is_empty() {
            let (mut level_ans, mut next_deque) = (vec![], vec![]);

            for node in deque {
                level_ans.push(node.borrow().val);
                if let Some(left) = node.borrow().left.clone() {
                    next_deque.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    next_deque.push(right);
                }
            }

            deque = next_deque;
            ans.push(level_ans);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::btree;

    #[test]
    fn p102_level_order_t1() {
        assert_eq!(
            Solution::p102_level_order(btree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
