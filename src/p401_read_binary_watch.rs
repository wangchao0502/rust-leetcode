#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p401_read_binary_watch(num: i32) -> Vec<String> {
        // codes
        let mut ans = vec![];

        fn count1(n: i32) -> i32 {
            let mut n = n;
            let mut count = 0;
            while n > 0 {
                count += n & 1;
                n >>= 1;
            }
            count
        }

        for i in 0..12 {
            for j in 0..60 {
                if count1(i) + count1(j) == num {
                    let ss = i.to_string() + ":";
                    if j < 10 {
                        ans.push(ss + "0" + &j.to_string());
                    } else {
                        ans.push(ss + &j.to_string());
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leetcode_prelude::vec_string;

    #[test]
    fn p401_read_binary_watch_t1() {
        assert_eq!(
            Solution::p401_read_binary_watch(1),
            vec_string![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"
            ]
        );
    }
}
