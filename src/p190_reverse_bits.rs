#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// 颠倒给定的 32 位无符号整数的二进制位。

// answers
impl Solution {
    pub fn p190_reverse_bits(x: u32) -> u32 {
        // codes
        let mut x = x;
        let mut y = 0;
        let mut power = 31;

        while x != 0 {
            y += (x & 1) << power;
            x >>= 1;
            power -= 1;
        }

        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p190_reverse_bits_t1() {
        assert_eq!(
            Solution::p190_reverse_bits(0b00000010100101000001111010011100u32),
            0b00111001011110000010100101000000u32
        );
    }
}
