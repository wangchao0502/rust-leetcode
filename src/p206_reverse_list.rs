#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p206_reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // codes
        let mut head = head;
        let mut tail = None;
        while let Some(mut n) = head {
            head = n.next;
            n.next = tail;
            tail = Some(n);
        }
        tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p206_reverse_list_t1() {
        assert_eq!(
            Solution::p206_reverse_list(linkedlist![1, 2, 3, 4]),
            linkedlist![4, 3, 2, 1]
        );
    }
}
