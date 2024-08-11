/*
https://leetcode.cn/problems/sqrtx/description/
69. x 的平方根 
给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。

 

示例 1：

输入：x = 4
输出：2
示例 2：

输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
 

提示：

0 <= x <= 231 - 1
 */

struct Solution;

impl Solution {
    // 解题思路，x的平方根的整数值，一定在 1...x的范围内
    // 使用二分查找法，满足 mid*mid<=x,则最接近 x的 mid即为答案
    // 注意mid*mid可能出现数据越界，使用除法 即mid<=x/mid，则可保证数据范围
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 1;
        let mut right = x;
        let mut result = 0;
        while left <= right {
            let mid = (left + right) / 2;
            if x / mid >= mid {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        result
    }
}