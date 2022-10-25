/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-25 13:12:36
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-25 14:40:17
 * @Description: 
 * 题目大意： 最长公共子序列
 * 给定一个长度为 n 的序列 A 和一个长度为 m 的序列 B（n,m < 5000），求出一个最长的序列，使得该序列既是 A 的子序列，也是 B 的子序列。
 * https://oi-wiki.org/dp/basic/#%E6%9C%80%E4%BC%98%E5%AD%90%E7%BB%93%E6%9E%84
 */
use std::cmp::max;

struct Solution1;

impl Solution1 {
    fn initDp(a: char, b: char) -> i32 {
        if a == b {
            return 1
        }
        0
    }
    fn solve(a: String, b: String) -> i32 {
        let a_vec = a.chars().collect::<Vec<_>>();
        let b_vec = b.chars().collect::<Vec<_>>();

        // 编程技巧，解决 i - 1 和 j - 1 需要 >= 0 的问题
        let m = a_vec.len() + 1;
        let n = b_vec.len() + 1;

        let mut dp = vec![vec![0; n]; m];

        for i in 1..m {
            for j in 1..n {
                if a_vec[i - 1] == b_vec[j - 1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[j - 1][i]);
                }
            }
        }
        0
    }
}

pub(crate) fn run() {
    let res = Solution1::solve("".to_string(), "".to_string());
    println!("basic_dp_01: {:?}", res);
}
