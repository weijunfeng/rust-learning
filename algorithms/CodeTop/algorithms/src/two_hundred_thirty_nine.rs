/*
https://leetcode.cn/problems/sliding-window-maximum/description/
239. 滑动窗口最大值
给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。

返回 滑动窗口中的最大值 。



示例 1：

输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
输出：[3,3,5,5,6,7]
解释：
滑动窗口的位置                最大值
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7
示例 2：

输入：nums = [1], k = 1
输出：[1]


提示：

1 <= nums.length <= 105
-104 <= nums[i] <= 104
1 <= k <= nums.length
 */
use std::collections::VecDeque;

struct Solution;

impl Solution {
    // 思路，使用一个队列存储可能成为最大值的索引
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut que = VecDeque::<usize>::new();
        let mut result = Vec::new();
        for (index, &val) in nums.iter().enumerate() {
            // 移除队列中不在当前窗口中的索引
            while let Some(&front) = que.front() {
                if index as i32 - (front as i32) >= k {
                    que.pop_front();
                } else {
                    break;
                }
            }
            // 移除队列中小于当前值的全部索引
            while let Some(&back) = que.back() {
                if nums[back] <= val {
                    que.pop_back();
                } else {
                    break;
                }
            }
            que.push_back(index);
            // 确保从第k个元素开始记录最大值
            if index as i32 >= k - 1 {
                if let Some(&front) = que.front() {
                    result.push(nums[front])
                }
            }
        }
        result
    }
    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 最大值索引
        let mut max_value_index: Option<usize> = None;
        let mut result = Vec::new();
        for (index, &val) in nums.iter().enumerate() {
            if let Some(value_index) = max_value_index {
                // 最大值的索引不在窗口内或当前值是最大值，则重置索引
                if index as i32 - (value_index as i32) >= k || nums[value_index] <= val {
                    max_value_index = None;
                }
            }
            if max_value_index.is_none() {
                max_value_index = Some(index);
            }
            // 确保从第k个元素开始记录最大值
            if index as i32 >= k - 1 {
                if let Some(value_index) = max_value_index {
                    result.push(nums[value_index])
                }
            }
        }
        result
    }
}