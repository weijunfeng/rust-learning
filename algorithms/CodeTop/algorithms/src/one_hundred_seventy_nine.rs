/*
https://leetcode.cn/problems/largest-number/description/
179. 最大数
中等
相关标签
相关企业
给定一组非负整数 nums，重新排列每个数的顺序（每个数不可拆分）使之组成一个最大的整数。

注意：输出结果可能非常大，所以你需要返回一个字符串而不是整数。

 

示例 1：

输入：nums = [10,2]
输出："210"
示例 2：

输入：nums = [3,30,34,5,9]
输出："9534330"
 

提示：

1 <= nums.length <= 100
0 <= nums[i] <= 109
 */
struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|&i| i.to_string()).collect::<Vec<String>>();
        // 按照两个数的字符拼接比较大小排序
        nums.sort_by(|a, b| {
            (b.clone() + a).cmp(&(a.clone() + &b))
        });
        nums.join("")
    }
}