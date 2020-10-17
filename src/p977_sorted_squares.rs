#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p977_sorted_squares(a: Vec<i32>) -> Vec<i32> {
        // codes
        let a2: Vec<i32> = a.iter().map(|&x| if x < 0 { -x } else { x }).collect();
        let (mut i, mut j) = (0, a.len() - 1);
        let mut result = vec![];

        while i <= j {
            if a2[i] < a2[j] {
                result.push(i32::pow(a2[j], 2));
                j -= 1;
            } else {
                result.push(i32::pow(a2[i], 2));
                i += 1;
            }
        }

        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p977_sorted_squares_t1() {
        assert_eq!(
            Solution::p977_sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn p977_sorted_squares_t2() {
        assert_eq!(
            Solution::p977_sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
