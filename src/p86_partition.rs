#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// problem description
// 给你一个链表和一个特定值 x ，请你对链表进行分隔，使得所有小于 x 的节点都出现在大于或等于 x 的节点之前。
// 你应当保留两个分区中每个节点的初始相对位置。

// answers
impl Solution {
    pub fn p86_partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // codes
        let mut head = head;
        let mut dh1 = Box::new(ListNode { val: 0, next: None });
        let mut dh1_mut = dh1.as_mut();
        let mut dh2 = Box::new(ListNode { val: 0, next: None });
        let mut dh2_mut = dh2.as_mut();

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                dh1_mut.next = Some(node);
                dh1_mut = dh1_mut.next.as_mut().unwrap();
            } else {
                dh2_mut.next = Some(node);
                dh2_mut = dh2_mut.next.as_mut().unwrap();
            }
        }

        dh1_mut.next = dh2.next;
        dh1.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p86_partition_t1() {
        assert_eq!(
            Solution::p86_partition(linkedlist![1, 4, 3, 2, 5, 2], 3),
            linkedlist![1, 2, 2, 4, 3, 5]
        );
    }
}
