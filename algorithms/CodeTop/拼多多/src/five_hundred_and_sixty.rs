/*
https://leetcode.cn/problems/subarray-sum-equals-k/description/
560. 和为 K 的子数组
给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。

子数组是数组中元素的连续非空序列。

 

示例 1：

输入：nums = [1,1,1], k = 2
输出：2
示例 2：

输入：nums = [1,2,3], k = 3
输出：2
 

提示：

1 <= nums.length <= 2 * 104
-1000 <= nums[i] <= 1000
-107 <= k <= 107
 */

struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
        let mut count = 0;
        let mut curr_sum = 0;
        for i in nums {
            curr_sum = curr_sum + i;
            sum.push_back(i);
            while curr_sum > k {
                curr_sum = curr_sum - sum.pop_front().unwrap();
            }
            if curr_sum == k {
                count = count + 1;
            }
        }
        count
    }
}