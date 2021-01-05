#![allow(dead_code)]

// use mods

pub struct Solution {}

// description
// 在一个由小写字母构成的字符串 s 中，包含由一些连续的相同字符所构成的分组。
// 例如，在字符串 s = "abbxxxxzyy" 中，就含有 "a", "bb", "xxxx", "z" 和 "yy" 这样的一些分组。
// 分组可以用区间 [start, end] 表示，其中 start 和 end 分别表示该分组的起始和终止位置的下标。
// 上例中的 "xxxx" 分组用区间表示为 [3,6] 。
// 我们称所有包含大于或等于三个连续字符的分组为 较大分组 。
// 找到每一个 较大分组 的区间，按起始位置下标递增顺序排序后，返回结果。

// answers
impl Solution {
    pub fn p830_large_group_positions(s: String) -> Vec<Vec<i32>> {
        // codes
        let len = s.len();

        if len < 3 {
            return vec![];
        }

        let mut ans = vec![];
        let mut left = 0;
        let mut right = 1;
        let chars = s.chars().collect::<Vec<char>>();

        while right < len {
            if chars[right] == chars[left] {
                // 继续往右找，到最大长度
                while right < len - 1 && chars[right + 1] == chars[right] {
                    right += 1;
                }
                if right - left >= 2 {
                    ans.push(vec![left as i32, right as i32]);
                }
            } else {
                left = right;
            }

            right += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p830_large_group_positions_t1() {
        assert_eq!(
            Solution::p830_large_group_positions("abbxxxxzzy".to_string()),
            vec![vec![3, 6]]
        );
    }

    #[test]
    fn p830_large_group_positions_t2() {
        assert_eq!(
            Solution::p830_large_group_positions("abcdddeeeeaabbbcd".to_string()),
            vec![vec![3, 5], vec![6, 9], vec![12, 14]]
        );
    }

    #[test]
    fn p830_large_group_positions_t3() {
        assert_eq!(
            Solution::p830_large_group_positions("abcdddeeeeaabbb".to_string()),
            vec![vec![3, 5], vec![6, 9], vec![12, 14]]
        );
    }
}
