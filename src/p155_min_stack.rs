#![allow(dead_code)]

// use mods

struct MinStack {
    stack: Vec<i32>,
    min_v: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_v: i32::MAX,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.is_empty() {
            self.min_v = x;
        } else if x <= self.min_v {
            self.stack.push(self.min_v);
            self.min_v = x;
        }
        self.stack.push(x);
    }

    // 单个栈，在最小值改变的时候把最小值也压入栈中
    fn pop(&mut self) {
        if let Some(v) = self.stack.pop() {
            if v == self.min_v {
                self.min_v = self.stack.pop().unwrap_or(i32::MAX);
            }
        }
    }

    fn top(&self) -> i32 {
        if let Some(v) = self.stack.last() {
            *v
        } else {
            i32::MAX
        }
    }

    fn get_min(&self) -> i32 {
        self.min_v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p155_min_stack_t1() {
        let mut obj = MinStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        obj.pop();
        assert_eq!(obj.top(), 1);
        assert_eq!(obj.get_min(), 1);
    }
}
