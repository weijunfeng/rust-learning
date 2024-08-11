/*
https://leetcode.cn/problems/string-to-integer-atoi/description/
8. 字符串转换整数 (atoi)
请你来实现一个 myAtoi(string s) 函数，使其能将字符串转换成一个 32 位有符号整数。

函数 myAtoi(string s) 的算法如下：

空格：读入字符串并丢弃无用的前导空格（" "）
符号：检查下一个字符（假设还未到字符末尾）为 '-' 还是 '+'。如果两者都不存在，则假定结果为正。
转换：通过跳过前置零来读取该整数，直到遇到非数字字符或到达字符串的结尾。如果没有读取数字，则结果为0。
舍入：如果整数数超过 32 位有符号整数范围 [−231,  231 − 1] ，需要截断这个整数，使其保持在这个范围内。具体来说，小于 −231 的整数应该被舍入为 −231 ，大于 231 − 1 的整数应该被舍入为 231 − 1 。
返回整数作为最终结果。

 

示例 1：

输入：s = "42"

输出：42

解释：加粗的字符串为已经读入的字符，插入符号是当前读取的字符。

带下划线线的字符是所读的内容，插入符号是当前读入位置。
第 1 步："42"（当前没有读入字符，因为没有前导空格）
         ^
第 2 步："42"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
         ^
第 3 步："42"（读入 "42"）
           ^
示例 2：

输入：s = " -042"

输出：-42

解释：

第 1 步："   -042"（读入前导空格，但忽视掉）
            ^
第 2 步："   -042"（读入 '-' 字符，所以结果应该是负数）
             ^
第 3 步："   -042"（读入 "042"，在结果中忽略前导零）
               ^
示例 3：

输入：s = "1337c0d3"

输出：1337

解释：

第 1 步："1337c0d3"（当前没有读入字符，因为没有前导空格）
         ^
第 2 步："1337c0d3"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
         ^
第 3 步："1337c0d3"（读入 "1337"；由于下一个字符不是一个数字，所以读入停止）
             ^
示例 4：

输入：s = "0-1"

输出：0

解释：

第 1 步："0-1" (当前没有读入字符，因为没有前导空格)
         ^
第 2 步："0-1" (当前没有读入字符，因为这里不存在 '-' 或者 '+')
         ^
第 3 步："0-1" (读入 "0"；由于下一个字符不是一个数字，所以读入停止)
          ^
示例 5：

输入：s = "words and 987"

输出：0

解释：

读取在第一个非数字字符“w”处停止。

 

提示：

0 <= s.length <= 200
s 由英文字母（大写和小写）、数字（0-9）、' '、'+'、'-' 和 '.' 组成
 */
struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut nums = Vec::new();
        let mut is_negative = false;
        for ch in s.chars() {
            if ch >= '0' && ch <= '9' {
                nums.push(ch.to_digit(10).unwrap());
                continue;
            }
            if nums.is_empty() {
                if ch == ' ' {
                    continue;
                }
                if ch == '-' || ch == '+' {
                    is_negative = ch == '-';
                    continue;
                }
            }
            break;
        }
        if nums.is_empty() {
            return 0;
        }
        let mut sum: i32 = 0;
        for &num in nums.iter() {
            if sum > (i32::MAX - num as i32) / 10 {
                return if is_negative { i32::MIN } else { i32::MAX };
            }
            sum = sum * 10 + num as i32;
        }
        if is_negative { -sum } else { sum }
    }

    pub fn my_atoi2(s: String) -> i32 {
        let mut ch = s.chars().peekable();
        let mut sign = 1;
        let mut result = 0;
        while let Some(num) = ch.peek() {
            if *num == ' ' {
                ch.next();
            } else {
                break;
            }
        }
        if let Some(num) = ch.peek() {
            if *num == '-' {
                sign = -1;
                ch.next();
            } else if *num == '+' {
                sign = 1;
                ch.next();
            }
        }
        while let Some(num) = ch.peek() {
            if let Some(digit) = num.to_digit(10) {
                // 注意处理数据越界点, 使用除法防止结果溢出
                if result > (i32::MAX - digit as i32) / 10 {
                    return if sign == -1 { i32::MIN } else { i32::MAX };
                }
                result = result * 10 + digit as i32;
            } else {
                break;
            }
            ch.next();
        }
        result * sign
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::Solution::my_atoi("42".to_string());
    }
}