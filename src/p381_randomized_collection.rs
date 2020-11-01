#![allow(dead_code)]

// use mods
use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
pub struct RandomizedCollection {
    map: HashMap<i32, i32>,
    total: i32,
}

// add structs

// answers
// https://leetcode-cn.com/problems/insert-delete-getrandom-o1-duplicates-allowed/solution/o1-shi-jian-cha-ru-shan-chu-he-huo-qu-sui-ji-yua-5/
// the remove function is not O(1)
impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let counter = self.map.entry(val).or_insert(0);
        self.total += 1;
        *counter += 1;
        *counter == 1
    }
    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.map.get(&val).is_none() || *self.map.get(&val).unwrap() == 0 {
            return false;
        }
        self.total -= 1;
        self.map.entry(val).and_modify(|x| *x -= 1);
        true
    }

    /** Get a random element from the collection. */
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0, self.total) + 1;
        println!("roll: [0, {}] - {}", self.total, roll);
        let mut sum = 0;
        for (val, count) in &self.map {
            sum += *count;
            if roll <= sum {
                return *val;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p381_randomized_collection_t1() {
        let mut rc = RandomizedCollection::new();
        assert_eq!(rc.insert(0), true);
        assert_eq!(rc.remove(0), true);
        assert_eq!(rc.insert(-1), true);
        assert_eq!(rc.remove(0), false);
        println!("{:?}", rc.map);
        println!("{}", rc.get_random());
    }
}
