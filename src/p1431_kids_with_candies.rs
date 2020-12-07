#![allow(dead_code)]

// use mods

pub struct Solution {}

// problem description
// Given the array candies and the integer extraCandies,
// where candies[i] represents the number of candies that the ith kid has.
// For each kid check if there is a way to distribute extraCandies among the kids such that he or
// she can have the greatest number of candies among them.
// Notice that multiple kids can have the greatest number of candies.

// answers
impl Solution {
    pub fn p1431_kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // codes
        let max_candies = *candies.iter().max().unwrap();
        candies.iter().map(|candy| candy + extra_candies >= max_candies).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1431_kids_with_candies_t1() {
        assert_eq!(
            Solution::p1431_kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn p1431_kids_with_candies_t2() {
        assert_eq!(
            Solution::p1431_kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }
}
