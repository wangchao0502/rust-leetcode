#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// add structs

// Given a singly linked list L: L0→L1→…→Ln-1→Ln,
// reorder it to: L0→Ln→L1→Ln-1→L2→Ln-2→…
// You may not modify the values in the list's nodes, only nodes itself may be changed.

// Example 1:
// Given 1->2->3->4, reorder it to 1->4->2->3.
// Example 2:
// Given 1->2->3->4->5, reorder it to 1->5->2->4->3.

// answers
// LinkList divide -> reverse -> merge
impl Solution {
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }

    fn reverse(
        head: Option<Box<ListNode>>,
        accumulator: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let tmp = node.next;
                node.next = accumulator;
                Self::reverse(tmp, Some(node))
            }
            None => accumulator,
        }
    }

    fn merge(
        mut left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
        count: usize,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            None
        } else {
            match (left.as_mut(), right.as_ref()) {
                (None, None) => None,
                (Some(_), None) => left,
                (None, Some(_)) => right,
                (Some(l), Some(_)) => {
                    l.next = Self::merge(right, l.next.take(), count - 1);
                    left
                }
            }
        }
    }

    pub fn p143_reorder_list(head: &mut Option<Box<ListNode>>) {
        // codes
        let len = Self::length(&head);
        let mut ptr = head.as_mut();

        for _ in 0..(len / 2) {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        if let Some(node) = ptr {
            let reverse = Self::reverse(node.next.take(), None);

            if let Some(node) = head {
                node.next = Self::merge(reverse, node.next.take(), len - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p143_reorder_list_t1() {
        let mut input = linkedlist![1, 2, 3, 4];
        Solution::p143_reorder_list(&mut input);
        assert_eq!(input, linkedlist![1, 4, 2, 3]);
    }
}
