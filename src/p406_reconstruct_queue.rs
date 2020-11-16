#![allow(dead_code)]

// use mods

pub struct Solution {}

// Suppose you have a random list of people standing in a queue.
// Each person is described by a pair of integers (h, k),
// where h is the height of the person and k is the number of people
// in front of this person who have a height greater than or equal to h.
// Write an algorithm to reconstruct the queue.
// Note:
// The number of people is less than 1,100.

// answers
// backtrack
impl Solution {
    pub fn p406_reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if people.len() <= 1 {
            people
        } else {
            people.sort_unstable_by_key(|x| ((!x[0]) as i64) << 32 | x[1] as i64);
            let mut d = people.drain(..);
            let mut new_people = vec![d.next().unwrap()];
            for i in d {
                new_people.insert(i[1] as usize, i)
            }
            new_people
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p406_reconstruct_queue_t1() {
        // [[5,0],[7,0],[6,1],[7,1],[5,2],[4,4]]
        // [[5,0],[7,0],[5,2],[6,1],[7,1],[4,4]]
        // [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
        assert_eq!(
            Solution::p406_reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }
}
