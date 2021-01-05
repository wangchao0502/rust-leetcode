#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// problem description
// 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
// k 是一个正整数，它的值小于或等于链表的长度。
// 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
//
// 示例：
// 给你这个链表：1->2->3->4->5
// 当 k = 2 时，应当返回: 2->1->4->3->5
// 当 k = 3 时，应当返回: 3->2->1->4->5
//
// 说明：
// 你的算法只能使用常数的额外空间。
// 你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。

// answers
// 模拟
impl Solution {
    fn reverse(
        mut head: Option<Box<ListNode>>,
        mut tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = tail.take();
            tail = Some(node);
        }
        // next_head
        tail
    }

    pub fn p25_reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // codes
        let mut head = head;
        let mut tail = &mut head;

        for _ in 0..k {
            if let Some(node) = tail.as_mut() {
                tail = &mut node.next;
            } else {
                // 长度小于k，停止翻转
                return head;
            }
        }

        let next_head = Self::p25_reverse_k_group(tail.take(), k);
        Self::reverse(head, next_head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p25_reverse_k_group_t1() {
        assert_eq!(
            Solution::p25_reverse_k_group(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![2, 1, 4, 3, 5]
        );
    }

    #[test]
    fn p25_reverse_k_group_t2() {
        assert_eq!(
            Solution::p25_reverse_k_group(linkedlist![1, 2, 3, 4, 5], 3),
            linkedlist![3, 2, 1, 4, 5]
        );
    }
}
