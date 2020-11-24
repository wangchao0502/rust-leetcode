#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// problem description
// Sort a linked list using insertion sort.

// answers
impl Solution {
    pub fn p147_insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // codes
        let mut head = head;
        let mut new_head = ListNode::new(0);
        while let Some(mut boxed) = head.take() {
            head = boxed.next.take();
            let mut ptr = &mut new_head;
            while ptr.next.as_ref().is_some() && ptr.next.as_ref().unwrap().val < boxed.val {
                ptr = ptr.next.as_mut().unwrap();
            }
            boxed.next = ptr.next.take();
            ptr.next = Some(boxed);
        }
        new_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p147_insertion_sort_list_t1() {
        assert_eq!(
            Solution::p147_insertion_sort_list(linkedlist![4, 2, 1, 3]),
            linkedlist![1, 2, 3, 4]
        );
    }
}
