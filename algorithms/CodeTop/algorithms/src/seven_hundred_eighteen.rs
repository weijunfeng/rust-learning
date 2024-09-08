/*
https://leetcode.cn/problems/maximum-length-of-repeated-subarray/description/
718. 最长重复子数组
给两个整数数组 nums1 和 nums2 ，返回 两个数组中 公共的 、长度最长的子数组的长度 。

 

示例 1：

输入：nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
输出：3
解释：长度最长的公共子数组是 [3,2,1] 。
示例 2：

输入：nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
输出：5
 

提示：

1 <= nums1.length, nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 100
 */

struct Solution;
impl Solution {
    // 动态规划，5部曲
    // 1. 确定dp数组及下标定义，dp[i][j] 表示以[i-1]为下标的nums1和以[j-1]为下标的nums2的最长子数组长度
    // 2. 确定递推公式，根据定义推导出来，dp[i][j] = dp[i-1][j-1]+1
    // 3. dp初始化，为 0
    // 4. 确定遍历顺序和启始位置
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut max_len = 0;
        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }
                max_len = max_len.max(dp[i][j]);
            }
        }
        max_len
    }
}