/*
https://leetcode.cn/problems/majority-element/description/
169. 多数元素
简单
相关标签
相关企业
给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

 

示例 1：

输入：nums = [3,2,3]
输出：3
示例 2：

输入：nums = [2,2,1,1,1,2,2]
输出：2
 

提示：
n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109
 

进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
 */
struct Solution;
impl Solution {
    // 摩尔投票算法，
    //假设你在投票选人 如果你和候选人（利益）相同，你就会给他投一票（count+1），
    // 如果不同，你就会踩他一下（count-1）当候选人票数为0（count=0）时，就换一个候选人，
    // 但因为和你利益一样的人占比超过了一半 不论换多少次 ，最后留下来的都一定是个和你（利益）相同的人
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = nums.first().unwrap();
        for i in nums.iter() {
            if count == 0 {
                candidate = i;
            }
            count = count + if i == candidate { 1 } else { -1 };
        }
        candidate.clone()
    }
}