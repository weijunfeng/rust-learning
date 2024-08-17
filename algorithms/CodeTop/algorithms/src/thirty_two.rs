/*
https://fanyi.baidu.com/mtpe-individual/multimodal?query=32&lang=zh2en&ext_channel=Aldtype
32. 最长有效括号
给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号
子串
的长度。

 

示例 1：

输入：s = "(()"
输出：2
解释：最长有效括号子串是 "()"
示例 2：

输入：s = ")()())"
输出：4
解释：最长有效括号子串是 "()()"
示例 3：

输入：s = ""
输出：0
 

提示：

0 <= s.length <= 3 * 104
s[i] 为 '(' 或 ')'
 */
use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut sub_parentheses = vec![];
        let mut parentheses = VecDeque::new();
        let mut max = 0;
        for char in s.chars() {
            if let Some(&temp) = parentheses.front() {
                if temp == '(' && char == ')' {
                    parentheses.pop_front();
                    sub_parentheses.push(temp);
                    sub_parentheses.push(char);
                    continue;
                }
            }
            if char == ')' {
                max = sub_parentheses.len().max(max);
                sub_parentheses.clear();
            }
            parentheses.push_front(char);
        }
        max = sub_parentheses.len().max(max);
        max as i32
    }
    pub fn longest_valid_parentheses2(s: String) -> i32 {
        let mut queue: VecDeque<i32> = VecDeque::new();
        // queue中存放未匹配的括号位置
        queue.push_front(-1);
        let mut max = 0;
        for (index, val) in s.chars().enumerate() {
            if val == '(' {
                // 如果 val是'(' 则将其位置入栈，等待下一个')'出栈
                queue.push_front(index as i32);
            } else {
                // val为')'，出栈第一个元素
                queue.pop_front();
                // 栈为空，则需要把当前位置作为新的未匹配的括号位置
                if queue.is_empty() {
                    queue.push_front(index as i32);
                } else {
                    // 不为空，使用当前位置和栈的第一个元素作为匹配到有效括号长度
                    // 保留最大值
                    max = max.max(index as i32 - queue.front().unwrap())
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::Solution::longest_valid_parentheses(")()())".to_string());
    }
}