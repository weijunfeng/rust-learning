/*
https://leetcode.cn/problems/longest-increasing-subsequence/description/

300. 最长递增子序列

给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的
子序列
。

 
示例 1：

输入：nums = [10,9,2,5,3,7,101,18]
输出：4
解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
示例 2：

输入：nums = [0,1,0,3,2,3]
输出：4
示例 3：

输入：nums = [7,7,7,7,7,7,7]
输出：1
 

提示：

1 <= nums.length <= 2500
-104 <= nums[i] <= 104
 

进阶：

你能将算法的时间复杂度降低到 O(n log(n)) 吗?
 */
struct Solution;
impl Solution {
    // dp实现， 假设dp[i]为第 i 个结尾的递增子序列长度,j 为 i 前一个递增子序列长度，
    // 则  nums[j]<nums[i]时dp[i] = max(dp[i],dp[j]+1)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut max = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1)
                }
            }
            max = max.max(dp[i])
        }
        max
    }
    
    // 贪心+二分， g[i]表示长度为 i+1个的最小上升子序列的末尾元素的最小值
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        let mut g = vec![];
        for ref i in nums {
            let result = g.binary_search(i);
            let index = match result {
                Ok(index) => {
                    index
                }
                Err(index) => {
                    index
                }
            };
            if index == g.len() {
                g.push(*i);
            } else {
                g[index] = *i;
            }
        }
        g.len() as i32
    }
}