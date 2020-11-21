#![allow(dead_code)]

// use mods
use crate::utils::list_node::ListNode;

pub struct Solution {}

// problem description
// Given the head of a linked list, return the list after sorting it in ascending order.
// Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?

// answers
impl Solution {
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }

    fn merge_two_list(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut p = &mut result;

        while l1.is_some() && l2.is_some() {
            let deref_val = |node: &Box<ListNode>| (*node).val;
            let digit1 = l1.as_ref().map_or(0, deref_val);
            let digit2 = l2.as_ref().map_or(0, deref_val);

            if digit1 <= digit2 {
                let tmp = l1.as_mut().unwrap().next.take();
                p.next = l1.take();
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                p.next = l2.take();
                l2 = tmp;
            }

            p = p.next.as_mut().unwrap();
        }

        if l1.is_none() {
            p.next = l2.take();
        } else {
            p.next = l1.take();
        }

        result.next
    }

    pub fn p148_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // codes
        let len = Self::length(&head);

        if len < 2 {
            return head;
        }

        let mut ptr = head.as_mut();

        for _ in 0..(len / 2) - 1 {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        let (right, left) = (ptr.unwrap().next.take(), head);

        Self::merge_two_list(Self::p148_sort_list(left), Self::p148_sort_list(right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::list_node::build_list_node;

    #[test]
    fn p148_sort_list_t1() {
        assert_eq!(
            Solution::p148_sort_list(build_list_node(&vec![4, 2, 1, 3])),
            build_list_node(&vec![1, 2, 3, 4])
        );
    }
}
