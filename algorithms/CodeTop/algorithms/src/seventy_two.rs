/*
https://leetcode.cn/problems/edit-distance/description/
72. 编辑距离
给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。

你可以对一个单词进行如下三种操作：

插入一个字符
删除一个字符
替换一个字符
 

示例 1：

输入：word1 = "horse", word2 = "ros"
输出：3
解释：
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
示例 2：

输入：word1 = "intention", word2 = "execution"
输出：5
解释：
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')
 

提示：

0 <= word1.length, word2.length <= 500
word1 和 word2 由小写英文字母组成
 */
struct Solution;
impl Solution {
    // 思路
    // 动态规划法，定义 dp[i][j]表示把 i个 word1字符转为 j 个 word2字符的最小步骤
    // 如果 word1[i]=word2[j]，dp[i][j] =dp[i-1][j-1]即无需增加步骤
    // 插入法 dp[i][j] =dp[i][j-1]+1
    // 删除法 dp[i][j] =dp[i-1][j]+1
    // 修改法 dp[i][j] =dp[i-1][j-1]+1
    // dp[i][j] 为三者的最小值
    // 初始值dp[i][0] = i, dp[0][j] = j;
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 0..word1.len() {
            dp[i][0] = i as i32;
        }
        for j in 0..word2.len() {
            dp[0][j] = j as i32;
        }
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if &word1[i - 1..i] == &word2[j - 1..j] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]);
                }
            }
        }
        dp[word1.len()][word2.len()]
    }
}