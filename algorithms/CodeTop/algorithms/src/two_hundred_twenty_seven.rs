/*
https://leetcode.cn/problems/basic-calculator-ii/description/
227. 基本计算器 II
给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。

整数除法仅保留整数部分。

你可以假设给定的表达式总是有效的。所有中间结果将在 [-231, 231 - 1] 的范围内。

注意：不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。

 

示例 1：

输入：s = "3+2*2"
输出：7
示例 2：

输入：s = " 3/2 "
输出：1
示例 3：

输入：s = " 3+5 / 2 "
输出：5
 

提示：

1 <= s.length <= 3 * 105
s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开
s 表示一个 有效表达式
表达式中的所有整数都是非负整数，且在范围 [0, 231 - 1] 内
题目数据保证答案是一个 32-bit 整数
 */

struct Solution;
impl Solution {
    // 使用栈保存数据，遇到操作符，则处理上个操作符，为+，则保存正数；为-，保存为负数；为*，取出最后一个数与当前数相乘；为/，取出最后一个数与当前数相除
    // 1. 使用一个栈来保存数据，处理操作符号，需要确定其优先级
    // 2. 遍历字符串s
    //  - 如果是数字，解析出完整的数字并将其入栈
    //  - 如果是操作符，则根据操作符的优先级处理是否立即执行计算，或者将操作符和数字入栈
    //  - 遇到括号，将括号中的表达式作为一个新的子问题，递归处理
    //3.当字符串遍历结束，栈中保存的则为需要求和的数据
    pub fn calculate2(s: String) -> i32 {
        let mut vec = Vec::new();
        let mut operate = '+';
        let mut number: i32 = 0;
        for (index, ch) in s.chars().enumerate() {
            if let Some(num) = ch.to_digit(10) {
                number = number * 10 + num as i32;
            }
            let is_operate = !ch.is_digit(10) && ch != ' ';
            if is_operate || index == s.len() - 1 {
                let result = Solution::cal(&mut vec, operate, number);
                vec.push(result);
                number = 0;
                if is_operate {
                    operate = ch;
                }
            }
        }
        vec.iter().sum()
    }

    pub fn cal(vec: &mut Vec<i32>, operate: char, number: i32) -> i32 {
        match operate {
            '+' => number,
            '-' => -number,
            '*' => {
                if let Some(prev) = vec.pop() {
                    prev * number
                } else {
                    number
                }
            }
            '/' => {
                if let Some(prev) = vec.pop() {
                    prev / number
                } else {
                    number
                }
            }
            _ => {
                panic!("Invalid operate operator:{operate}")
            }
        }
    }

    pub fn calculate(s: String) -> i32 {
        let mut vec = Vec::new();
        let mut num = String::new();
        for (index, ch) in s.chars().enumerate() {
            if !Solution::is_operate(ch) && ch != ' ' {
                num.push(ch);
            }
            if Solution::is_operate(ch) || index == s.len() - 1 {
                let mut curr_num = num.clone();
                num.clear();
                if let Some(v) = vec.last() {
                    if v == "*" || v == "/" {
                        let operate: String = vec.pop().unwrap();
                        let num1: String = vec.pop().unwrap();
                        curr_num = Solution::calculate1(&num1, &operate, &curr_num);
                    }
                }
                vec.push(curr_num);
            }
            if Solution::is_operate(ch) {
                vec.push(ch.to_string());
            }
        }
        let mut result = 0;
        while !vec.is_empty() {
            let num2: String = vec.pop().unwrap();
            if vec.is_empty() {
                return num2.parse::<i32>().unwrap();
            }
            let operate: String = vec.pop().unwrap();
            let num1: String = vec.pop().unwrap();
            result = result + Solution::calculate1(&num1, &operate, &num2).parse::<i32>().unwrap();
        }
        result
    }

    pub fn is_operate(ch: char) -> bool {
        ch == '+' || ch == '-' || ch == '*' || ch == '/'
    }

    pub fn calculate1(num1: &str, operate: &str, num2: &str) -> String {
        match operate {
            "+" => {
                let result = num1.parse::<i32>().unwrap() + num2.parse::<i32>().unwrap();
                result.to_string()
            }
            "-" => {
                let result = num1.parse::<i32>().unwrap() - num2.parse::<i32>().unwrap();
                result.to_string()
            }
            "*" => {
                let result = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
                result.to_string()
            }
            "/" => {
                let result = num1.parse::<i32>().unwrap() / num2.parse::<i32>().unwrap();
                result.to_string()
            }
            _ => {
                panic!("Invalid operate operator:{operate}");
            }
        }
    }
}