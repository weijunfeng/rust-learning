/*
https://leetcode.cn/problems/coin-change/description/
322. 零钱兑换
给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。

计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。

你可以认为每种硬币的数量是无限的。

 

示例 1：

输入：coins = [1, 2, 5], amount = 11
输出：3 
解释：11 = 5 + 5 + 1
示例 2：

输入：coins = [2], amount = 3
输出：-1
示例 3：

输入：coins = [1], amount = 0
输出：0
 

提示：

1 <= coins.length <= 12
1 <= coins[i] <= 231 - 1
0 <= amount <= 104
 */

struct Solution;

impl Solution {
    // 动态规划实现：定义状态、定义初始值、状态转移方程
    // 定义状态：dp[i]标识组成金额 i所需的最小硬币数
    // 定义初始值：dp[0]=0, dp[i]=i32::MAX
    // 状态转移方程：对于每个金额 i，可以从所有硬币中选择一个 coin,如果选择了 coin,则 dp[i]可以 dp[i-coin]转移过来，dp[i] = min(dp[i],d[i-coin]+1)
    // 方程含义：表示当前金额 i的最少硬币数等于 i-coin金额的最少硬币数加上这个硬币即+1
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for &coin in coins.iter() {
                if i >= coin {
                    let i_usize = i as usize;
                    dp[i_usize] = dp[i_usize].min(dp[i_usize - coin as usize] + 1);
                }
            }
        }
        let result = dp[amount as usize];
        if result == amount + 1 {
            -1
        } else {
            result
        }
    }
}