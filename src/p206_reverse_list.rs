#![allow(dead_code)]

// use mods
use crate::utils::list_node::ListNode;

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
    use crate::utils::list_node::build_list_node;

    #[test]
    fn p206_reverse_list_t1() {
        assert_eq!(
            Solution::p206_reverse_list(build_list_node(&vec![1, 2, 3, 4])),
            build_list_node(&vec![4, 3, 2, 1])
        );
    }
}
