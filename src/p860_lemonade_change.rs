#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// At a lemonade stand, each lemonade costs $5.
// Customers are standing in a queue to buy from you, and order one at a time (in the order specified by bills).
// Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill.
// You must provide the correct change to each customer, so that the net transaction is that the customer pays $5.
// Note that you don't have any change in hand at first.
// Return true if and only if you can provide every customer with correct change.

// answers
impl Solution {
    pub fn p860_lemonade_change(bills: Vec<i32>) -> bool {
        // codes
        let mut five = 0;
        let mut ten = 0;

        for bill in bills {
            match bill {
                5 => {
                    five += 1;
                }
                10 => {
                    if five == 0 {
                        return false;
                    }
                    five -= 1;
                    ten += 1;
                }
                20 => {
                    if ten > 0 {
                        if five == 0 {
                            return false;
                        }
                        five -= 1;
                        ten -= 1;
                    } else {
                        if five < 3 {
                            return false;
                        }
                        five -= 3;
                    }
                }
                _ => {}
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p860_lemonade_change_t1() {
        assert_eq!(Solution::p860_lemonade_change(vec![5, 5, 5, 10, 20]), true);
    }
}
