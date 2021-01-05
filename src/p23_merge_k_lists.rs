#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// problem description
// 给你一个链表数组，每个链表都已经按升序排列。
// 请你将所有链表合并到一个升序链表中，返回合并后的链表。

// answers
// k heap, k = lists.len()
impl Solution {
    pub fn p23_merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // codes
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        let mut ans = Box::new(ListNode::new(0));
        let mut head = &mut ans;

        for list in lists {
            let mut plist = &list;
            while let Some(node) = plist {
                heap.push(Reverse(node.val));
                plist = &node.next;
            }
        }

        while let Some(Reverse(n)) = heap.pop() {
            head.next = Some(Box::new(ListNode::new(n)));
            head = head.next.as_mut().unwrap();
        }

        ans.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p23_merge_k_lists_t1() {
        assert_eq!(
            Solution::p23_merge_k_lists(vec![
                linkedlist![1, 4, 5],
                linkedlist![1, 3, 4],
                linkedlist![2, 6]
            ]),
            linkedlist![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }
}
