#![allow(dead_code)]

// use mods

pub struct Solution {}

// There are N gas stations along a circular route, where the amount of gas at station i is gas[i].
// You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from
// station i to its next station (i+1). You begin the journey with an empty tank at one of the gas stations.
// Return the starting gas station's index if you can travel around the circuit once in the clockwise direction,
// otherwise return -1.
//
// Note:
//
// If there exists a solution, it is guaranteed to be unique.
// Both input arrays are non-empty and have the same length.
// Each element in the input arrays is a non-negative integer.

// answers
impl Solution {
    pub fn p134_can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // codes
        let mut spare: i32 = 0;
        let mut min = i32::MAX;
        let mut index = 0;

        for i in 0..gas.len() {
            spare += gas[i] - cost[i];
            if spare < min {
                min = spare;
                index = i as i32;
            }
        }

        if spare < 0 {
            -1
        } else {
            (index + 1) % gas.len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p134_can_complete_circuit_t1() {
        assert_eq!(
            Solution::p134_can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn p134_can_complete_circuit_t2() {
        assert_eq!(
            Solution::p134_can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
