#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// problem description

// answers
impl Solution {
    pub fn p21_merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // codes
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::p21_merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::p21_merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p21_merge_two_lists_t1() {
        assert_eq!(
            Solution::p21_merge_two_lists(linkedlist![1, 2, 4], linkedlist![1, 3, 4]),
            linkedlist![1, 1, 2, 3, 4, 4]
        );
    }
}
