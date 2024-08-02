/*
https://leetcode.cn/problems/maximum-subarray/description/
53. 最大子数组和
给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

子数组
是数组中的一个连续部分。

 

示例 1：

输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
输出：6
解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
示例 2：

输入：nums = [1]
输出：1
示例 3：

输入：nums = [5,4,-1,7,8]
输出：23
 

提示：

1 <= nums.length <= 105
-104 <= nums[i] <= 104
 

进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
 */
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 前 i个位置的数据和，该和最小应该为 0，因为负数加任何数据都不会最大的和
        let mut sum = vec![0; nums.len()];
        sum[0] = nums[0];
        let mut ans = nums[0];
        for index in 1..nums.len() {
            // 每一次当前 index只有加大于 0 的前面和才会使用当前和增大
            sum[index] = sum[index - 1].max(0) + nums[index];
            ans = ans.max(sum[index]);
        }
        ans
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test() {
        let result = super::Solution::max_sub_array(vec![-2, -3, -5]);
        println!("result:{result}")
    }
}