/*
https://leetcode.cn/problems/next-permutation/description/
31. 下一个排列

整数数组的一个 排列  就是将其所有成员以序列或线性顺序排列。

例如，arr = [1,2,3] ，以下这些都可以视作 arr 的排列：[1,2,3]、[1,3,2]、[3,1,2]、[2,3,1] 。
整数数组的 下一个排列 是指其整数的下一个字典序更大的排列。更正式地，如果数组的所有排列根据其字典顺序从小到大排列在一个容器中，那么数组的 下一个排列 就是在这个有序容器中排在它后面的那个排列。如果不存在下一个更大的排列，那么这个数组必须重排为字典序最小的排列（即，其元素按升序排列）。

例如，arr = [1,2,3] 的下一个排列是 [1,3,2] 。
类似地，arr = [2,3,1] 的下一个排列是 [3,1,2] 。
而 arr = [3,2,1] 的下一个排列是 [1,2,3] ，因为 [3,2,1] 不存在一个字典序更大的排列。
给你一个整数数组 nums ，找出 nums 的下一个排列。

必须 原地 修改，只允许使用额外常数空间。

 

示例 1：

输入：nums = [1,2,3]
输出：[1,3,2]
示例 2：

输入：nums = [3,2,1]
输出：[1,2,3]
示例 3：

输入：nums = [1,1,5]
输出：[1,5,1]
 

提示：

1 <= nums.length <= 100
0 <= nums[i] <= 100
 */

struct Solution;

impl Solution {
    // 思路，题目排列，是从最后一个元素开始，依次取一个数，与前面的数组成排列
    // 排列后的下一个字典序更大的排列，一定是从最后一个元素开始，找到第一次出现递减的数与其后的最大一个数交换；然后对其后的数进行反转所得
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        // 找到第一个递减的位置
        let mut i = (nums.len() - 2) as i32;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i = i - 1;
        }
        if i > 0 {
            // 找到第一个比 i 位置大的数
            let mut j = nums.len() - 1;
            while j > 0 && nums[i as usize] >= nums[j] {
                j = j - 1;
            }
            nums.swap(i as usize, j);
        }
        let _temp = &nums[(i + 1) as usize..].reverse();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut data = vec![1, 2, 3];
        super::Solution::next_permutation(&mut data);
        println!("{data:?}")
    }
}