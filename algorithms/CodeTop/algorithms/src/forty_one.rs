/*
https://leetcode.cn/problems/first-missing-positive/description/
41. 缺失的第一个正数
给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
 

示例 1：

输入：nums = [1,2,0]
输出：3
解释：范围 [1,2] 中的数字都在数组中。
示例 2：

输入：nums = [3,4,-1,1]
输出：2
解释：1 在数组中，但 2 没有。
示例 3：

输入：nums = [7,8,9,11,12]
输出：1
解释：最小的正数 1 没有出现。
 

提示：

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
 */
struct Solution;
impl Solution {
    // 0..nums.len存放的值对应为索引+1，其他重置为 0
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        // nums中存放数如果小于 0或大于 nums.len()则重置 为0
        let mut nums = nums.iter().map(|&f| {
            if f < 0 || f > nums.len() as i32 {
                0usize
            } else {
                f as usize
            }
        }).collect::<Vec<usize>>();
        // 重置 nums对应索引位置的值为索引+1或 0
        for i in 0..nums.len() {
            while nums[i] != 0 && i + 1 != nums[i] {
                let k = nums[i] - 1;
                if nums[k] == nums[i] {
                    nums[i] = 0;
                    break;
                }
                nums.swap(i, k);
            }
        }
        // 取 nums中第一个为 0 值的索引，索引+1即为预期的值
        for (index, &val) in nums.iter().enumerate() {
            if val == 0 {
                return (index + 1) as i32;
            }
        }

        (nums.len() + 1) as i32
    }

    // 将值为 x(>0&&<=n)的元素放到 x-1的索引上，第一个不满足 num[i] = i+1的索引 i 对应的值即为预期值
    pub fn first_missing_positive2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            // 继续交换直到当前数字在正确位置上
            let mut x = nums[i];
            while x > 0 && (x as usize) <= n && x != nums[x as usize - 1] {
                let target_index = x as usize - 1;
                nums.swap(i, target_index);
                x = nums[i];
            }
        }

        // 找到第一个值不正确的索引
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        // 如果所有位置都正确，则最小缺失正整数是 n + 1
        (n as i32) + 1
    }
}