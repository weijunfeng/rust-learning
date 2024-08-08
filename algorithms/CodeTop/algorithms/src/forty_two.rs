/*
https://leetcode.cn/problems/trapping-rain-water/description/
42. 接雨水
给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

 

示例 1：



输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
输出：6
解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 
示例 2：

输入：height = [4,2,0,3,2,5]
输出：9
 

提示：

n == height.length
1 <= n <= 2 * 104
0 <= height[i] <= 105
 */
struct Solution;
impl Solution {
    // 解题思路，双指针法，直到两者相遇
    // 比较两指针对应的柱子高度，选择低点作为参考，如果左边低，则移动左边，右边低则移动右边
    // 所接雨水为左或右边的最大值减当值的累计和
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut left_max = i32::MIN;
        let mut right_max = i32::MIN;
        let mut water = 0;
        let mut right = height.len() - 1;
        while left < right {
            if height[left] < height[right] {
                if height[left] > left_max {
                    left_max = height[left];
                } else {
                    water = water + left_max - height[left];
                }
                left = left + 1;
            } else {
                if height[right] > right_max {
                    right_max = height[right];
                } else {
                    water = water + right_max - height[right];
                }
                right = right - 1;
            }
        }
        water
    }
}