#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// add structs

// answers
// two pointer
// dummy head -> virtual fake head
impl Solution {
    pub fn p19_remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // codes
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        // p2 is a mutable pointer to heap space with a copy value of ListNode
        let mut p2 = dummy.clone();
        // p1 is a mutable pointer to heap space
        let mut p1 = dummy.as_mut();

        // p2 go n steps
        for _ in 0..n {
            // unwrap -> Box<ListNode> or null
            p2 = p2.next.unwrap();
        }

        while p2.next.is_some() {
            p2 = p2.next.unwrap();
            p1 = p1.next.as_mut().unwrap();
        }

        let next = p1.next.as_mut().unwrap();
        p1.next = next.next.clone();

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p19_remove_nth_from_end_t1() {
        assert_eq!(
            Solution::p19_remove_nth_from_end(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![1, 2, 3, 5]
        );
    }
}
