#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p1024_video_stitching(mut clips: Vec<Vec<i32>>, t: i32) -> i32 {
        // codes
        clips.sort();
        let mut count = 0;
        let mut nxt = 0;
        let mut cur = 0;
        // println!("{:?}", clips);

        for v in &clips {
            let (l, r) = (v[0], v[1]);
            if l > nxt {
                return -1;
            }
            if l > cur {
                count += 1;
                cur = nxt;
            }
            nxt = nxt.max(r);
            if nxt >= t {
                return count + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1024_video_stitching_t1() {
        assert_eq!(
            Solution::p1024_video_stitching(
                vec![
                    vec![0, 2],
                    vec![4, 6],
                    vec![8, 10],
                    vec![1, 9],
                    vec![1, 5],
                    vec![5, 9]
                ],
                10
            ),
            3
        );
    }
}
