#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// add structs

// answers
// link list -> recursion
impl Solution {
    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
        mut val: i32,
        mut ln: ListNode,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && val == 0 {
            return None;
        }
        if let Some(n1) = l1 {
            val += n1.val;
            l1 = n1.next;
        }
        if let Some(n2) = l2 {
            val += n2.val;
            l2 = n2.next;
        }
        ln.val = if val > 9 { val - 10 } else { val };
        ln.next = Self::merge(l1, l2, val / 10, ListNode::new(-1));
        Some(Box::new(ln))
    }

    pub fn p2_add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::merge(l1, l2, 0, ListNode::new(-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p2_add_two_numbers_t1() {
        assert_eq!(
            Solution::p2_add_two_numbers(linkedlist![2, 4, 3], linkedlist![5, 6, 4]),
            linkedlist![7, 0, 8]
        );
    }
}
