/*
https://leetcode.cn/problems/subsets/description/
78. 子集
给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的
子集
（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。

 

示例 1：

输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
示例 2：

输入：nums = [0]
输出：[[],[0]]
 

提示：

1 <= nums.length <= 10
-10 <= nums[i] <= 10
nums 中的所有元素 互不相同
 */
struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Self::sub_set(0, &nums, &mut vec![], &mut result);
        result.sort_by_key(|f|f.len());
        result
    }

    pub fn sub_set(index: usize, nums: &Vec<i32>, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            result.push(curr.clone());
            return;
        }
        curr.push(nums[index]);
        Self::sub_set(index + 1, nums, curr, result);
        curr.remove(curr.len() - 1);
        Self::sub_set(index + 1, nums, curr, result);
    }

    pub fn subsets2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(0, &nums, &mut vec![], &mut result);
        result
    }

    // 递归回溯
    pub fn backtrack(start: usize, nums: &Vec<i32>, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(curr.clone()); // 每到一个节点都添加当前子集
        for i in start..nums.len() {
            curr.push(nums[i]); // 做选择
            Self::backtrack(i + 1, nums, curr, result); // 递归
            curr.pop(); // 撤销选择
        }
    }

    // 迭代处理
    pub fn subsets3(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        // 从空集开始，把每个元素添加到现有子集的副本中，生成新的子集
        result.push(vec![]);
        for num in nums {
            let size = result.len();
            for i in 0..size {
                let mut new_sub = result[i].clone();
                new_sub.push(num);
                result.push(new_sub);
            }
        }
        result
    }
}