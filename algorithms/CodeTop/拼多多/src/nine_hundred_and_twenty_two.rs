/*
https://leetcode.cn/problems/sort-array-by-parity-ii/description/
922. 按奇偶排序数组 II
给定一个非负整数数组 nums，  nums 中一半整数是 奇数 ，一半整数是 偶数 。

对数组进行排序，以便当 nums[i] 为奇数时，i 也是 奇数 ；当 nums[i] 为偶数时， i 也是 偶数 。

你可以返回 任何满足上述条件的数组作为答案 。

 

示例 1：

输入：nums = [4,2,5,7]
输出：[4,5,2,7]
解释：[4,7,2,5]，[2,5,4,7]，[2,7,4,5] 也会被接受。
示例 2：

输入：nums = [2,3]
输出：[2,3]
 

提示：

2 <= nums.length <= 2 * 104
nums.length 是偶数
nums 中一半是偶数
0 <= nums[i] <= 1000
 

进阶：可以不使用额外空间解决问题吗？
 */
struct Solution;
impl Solution {
    // 循环处理保证偶数位数值是偶数，奇数位数值自然是奇数
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut even_index = 0;
        let mut odd_number_index = 1;
        while even_index < nums.len() && odd_number_index < nums.len() {
            if nums[even_index] % 2 == 1 {
                let a = nums[even_index];
                nums[even_index] = nums[odd_number_index];
                nums[odd_number_index] = a;
                odd_number_index = odd_number_index + 2;
            } else {
                even_index = even_index + 2;
            }
        }
        nums
    }
}