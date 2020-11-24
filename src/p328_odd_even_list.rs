#![allow(dead_code)]

// use mods
use leetcode_prelude::ListNode;

pub struct Solution {}

// Given a singly linked list, group all odd nodes together followed by the even nodes.
// Please note here we are talking about the node number and not the value in the nodes.
// You should try to do it in place.
// The program should run in O(1) space complexity and O(nodes) time complexity.

// answers
impl Solution {
    pub fn p328_odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd = head?;
        if odd.next.is_none() {
            return Some(odd);
        }
        let mut even = odd.next.take().unwrap();
        let (mut po, mut pe) = (&mut odd, &mut even);

        while pe.next.is_some() {
            po.next = pe.next.take();
            po = po.next.as_mut().unwrap();

            if po.next.is_none() {
                break;
            }

            pe.next = po.next.take();
            pe = pe.next.as_mut().unwrap();
        }
        po.next = Some(even);
        Some(odd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::linkedlist;

    #[test]
    fn p328_odd_even_list_t1() {
        assert_eq!(
            Solution::p328_odd_even_list(linkedlist![1, 2, 3, 4, 5]),
            linkedlist![1, 3, 5, 2, 4]
        );
    }

    #[test]
    fn p328_odd_even_list_t2() {
        assert_eq!(
            Solution::p328_odd_even_list(linkedlist![2, 1, 3, 5, 6, 4, 7]),
            linkedlist![2, 3, 6, 7, 1, 5, 4]
        );
    }

    #[test]
    fn p328_odd_even_list_t3() {
        assert_eq!(
            Solution::p328_odd_even_list(linkedlist![2]),
            linkedlist![2]
        );
    }
}
