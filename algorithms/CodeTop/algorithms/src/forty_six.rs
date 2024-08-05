/*
https://leetcode.cn/problems/permutations/description/
46. 全排列
给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

 

示例 1：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
示例 2：

输入：nums = [0,1]
输出：[[0,1],[1,0]]
示例 3：

输入：nums = [1]
输出：[[1]]
 

提示：

1 <= nums.length <= 6
-10 <= nums[i] <= 10
nums 中的所有整数 互不相同
 */
/*
回溯（Backtracking）算法是一种用于解决组合问题、排列问题和其他复杂问题的递归算法。它通过尝试构造解的所有可能性来找出符合条件的解。回溯算法的核心思想是逐步构建解的过程，在构建过程中如果发现当前解不能满足条件，就撤销最后一步，回到上一个步骤继续尝试其他可能性。

基本概念

	1.	选择：在每一步选择一个候选解。
	2.	约束：检查当前选择是否符合问题的约束条件。
	3.	回溯：如果当前选择不符合条件，撤销该选择，并尝试其他候选解。

回溯算法的基本步骤

	1.	定义递归函数：
	•	递归函数通常会接收当前状态作为参数，逐步构建解。
	•	如果当前状态符合问题的要求，将其加入结果集中。
	•	如果当前状态不符合要求，进行回溯操作，撤销上一步选择。
	2.	递归调用：
	•	在递归函数中，进行下一步的选择。
	•	递归调用将尝试所有可能的选择。
	3.	撤销操作：
	•	在递归返回后，撤销当前选择，恢复到之前的状态。
 */
struct Solution;
impl Solution {
    // 解题思路：取每一个位置值，与其他位置组成数组，使用递归处理，递归时需处理已经使用过的位置
    // 也可粗暴些，使用 nums.len层的遍历
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::permute_recursion(&nums, &mut result, &mut Vec::new(), &mut vec![false; nums.len()]);
        result
    }

    pub fn permute_recursion(nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, used: &mut Vec<bool>) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            Self::permute_recursion(nums, result, path, used);
            used[i] = false;
            path.pop();
        }
    }
}