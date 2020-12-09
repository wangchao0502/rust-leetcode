#![allow(dead_code)]

// use mods
use std::collections::VecDeque;

// problem description
// Given a nested list of integers, implement an iterator to flatten it.
// Each element is either an integer, or a list -- whose elements may also be integers or other lists.

// answers
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    nested_list: VecDeque<NestedInteger>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            nested_list: nested_list.into_iter().collect(),
        }
    }

    fn next(&mut self) -> i32 {
        if let NestedInteger::Int(n) = self.nested_list.pop_front().unwrap() {
            n
        } else {
            0
        }
    }

    fn has_next(&mut self) -> bool {
        while !self.nested_list.is_empty() {
            if let Some(nested) = self.nested_list.pop_front() {
                match nested {
                    NestedInteger::List(list) => {
                        for item in list.into_iter().rev() {
                            self.nested_list.push_front(item);
                        }
                    }
                    NestedInteger::Int(n) => {
                        self.nested_list.push_front(NestedInteger::Int(n));
                        break;
                    }
                }
            }
        }

        !self.nested_list.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p341_nested_iterator_t1() {
        let mut obj = NestedIterator::new(vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ]);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 2);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), false);
    }

    #[test]
    fn p341_nested_iterator_t2() {
        let mut obj = NestedIterator::new(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ]);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 4);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 6);
        assert_eq!(obj.has_next(), false);
    }
}
