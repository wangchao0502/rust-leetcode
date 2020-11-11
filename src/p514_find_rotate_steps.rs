#![allow(dead_code)]

// use mods

pub struct Solution {}

// add structs

// answers
impl Solution {
    pub fn p514_find_rotate_steps(ring: String, key: String) -> i32 {
        let (ring, key) = (ring.into_bytes(), key.into_bytes());
        let n = ring.len();
        let m = key.len();
        // 解释m为什么要+1，因为m表示key的下标值
        let mut dp = vec![vec![std::usize::MAX; n]; m + 1];
        // 因为状态转移方程/dp填表依赖当前行的下一行的值，初始填表时倒数第二行，所以除最后一行置零以外全部初始化为最大值
        for j in 0..n {
            dp[m][j] = 0;
        }
        for i in (0..m).rev() {
            for j in 0..n {
                // dp[i][j]表示转盘已经对完key的前i个字母后，穷举转盘12点钟方向可能指向的任意位置j
                // 我们可以假设转盘对完上个字母a(key[i])后，可以调皮的随意多移动到字母b，反正并不是每步都要最优解总体才是最优解
                // i: 转盘下一步要拨到key[i]的字母
                // j: 枚举转盘当前步的所有位置
                // k: 枚举转盘可以达到下一步key[i]字母的目标位置
                for k in 0..n {
                    // 可以参考leetcode官方题解，优化状态转移层，用类似Python的collections.Counter去统计每个小写字母在ring中出现在哪些索引位置
                    // for &k in ring_counter.get(&key[i]).unwrap()，由于ring全为小写字母，所以Counter用数组会比Hashmap性能更好
                    if ring[k] == key[i] {
                        // 从穷举的当前位置j，到转盘其中一个字母等于key[i]的位置k，所需要的正转/反转的步数
                        let diff = if j < k { k - j } else { j - k };
                        // 选正转或反转的步数最小值
                        let step = diff.min(n - diff);
                        // 状态转移方程: dp[i][j]=dp[i][j(ring[k]=key[i+1])].min(dp[i+1][k]+j到k正转或反转的最小步数)
                        dp[i][j] = dp[i][j].min(dp[i + 1][k] + step);
                    }
                }
            }
        }
        // 解释+m: 转盘对准了key的m个字母后，key的m个字母各需要一个步数去拨号，所以要+m
        (dp[0][0] + m) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p514_find_rotate_steps_t1() {
        assert_eq!(
            Solution::p514_find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
    }

    #[test]
    fn p514_find_rotate_steps_t2() {
        assert_eq!(
            // i o t f o i o t f o i o t f o
            Solution::p514_find_rotate_steps("iotfo".to_string(), "fioot".to_string()),
            11
        );
    }
}
