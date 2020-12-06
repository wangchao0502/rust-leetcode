#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// You are given an m x n integer grid accounts where accounts[i][j] is
// the amount of money the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank.
// Return the wealth that the richest customer has.
// A customer's wealth is the amount of money they have in all their bank accounts.
// The richest customer is the customer that has the maximum wealth.

// answers
impl Solution {
    pub fn p1672_maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        // codes
        accounts.iter().map(|x| x.iter().sum()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1672_maximum_wealth_t1() {
        assert_eq!(
            Solution::p1672_maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }

    #[test]
    fn p1672_maximum_wealth_t2() {
        assert_eq!(
            Solution::p1672_maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }
}
