/*
https://leetcode.cn/problems/multiply-strings/description/
43. 字符串相乘
给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。

注意：不能使用任何内置的 BigInteger 库或直接将输入转换为整数。



示例 1:

输入: num1 = "2", num2 = "3"
输出: "6"
示例 2:

输入: num1 = "123", num2 = "456"
输出: "56088"


提示：

1 <= num1.length, num2.length <= 200
num1 和 num2 只能由数字组成。
num1 和 num2 都不包含任何前导零，除了数字0本身。
 */
struct Solution;
impl Solution {
    // 解题思路:模拟两数相乘
    pub fn multiply(num1: String, num2: String) -> String {
        // 两数相乘后的长度，不可能比这两个字符串的长度和长
        // result高位表示num1和num2中每个数相乘后的个位数，次高位标识进位数
        let mut result = vec![0; num1.len() + num2.len()];
        // 从右向左处理乘数，即从低位到高位相乘
        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                // i和 j位置的乘数需要加上上一次的进位数，即 i+j+1的位置
                let multiply = &num1[i..i + 1].parse::<i32>().unwrap()
                    * &num2[j..j + 1].parse::<i32>().unwrap()
                    + result[i + j + 1];
                // 使用 i+j+1的位置存放 i和 j 相乘后的个位数
                // 使用 i+j的位置存放 i和 j 相乘后的进位数
                result[i + j + 1] = multiply % 10;
                result[i + j] += multiply / 10;
            }
        }
        let mut result_str = String::new();
        for i in result {
            // 转为字符串，需要排除 result中前部未被处理的 0 值
            if !(result_str.is_empty() && i == 0) {
                result_str.push_str(&i.to_string());
            }
        }
        result_str
    }
}
