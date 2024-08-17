/*
https://leetcode.cn/problems/minimum-window-substring/description/
76. 最小覆盖子串
给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。

 

注意：

对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
如果 s 中存在这样的子串，我们保证它是唯一的答案。
 

示例 1：

输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"
解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
示例 2：

输入：s = "a", t = "a"
输出："a"
解释：整个字符串 s 是最小覆盖子串。
示例 3:

输入: s = "a", t = "aa"
输出: ""
解释: t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。
 

提示：

m == s.length
n == t.length
1 <= m, n <= 105
s 和 t 由英文字母组成
 

进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？
 */
use std::collections::HashMap;
use std::usize;

struct Solution;
impl Solution {
    // 滑动窗口解法
    // 先定义一个 need标识 t 中字符出现的次数
    // 再定义一个 window标识 left到 right窗口内字符出现次数，
    // 先扩张 right到 window记录包含全部的 need,则找到一个窗口，再扩张 left，找到最小的子串的开始位置和长度
    pub fn min_window(s: String, t: String) -> String {
        let mut need: HashMap<char, i32> = HashMap::new();
        let mut window: HashMap<char, i32> = HashMap::new();
        let mut left = 0;
        let mut right = 0;
        let mut valid = 0;
        let mut len = usize::MAX;
        let mut start = 0;
        // 统计 t 中字符次数
        for ch in t.chars() {
            need.entry(ch).and_modify(|f| { *f = *f + 1 }).or_insert(1);
        }
        let s_char: Vec<char> = s.chars().collect();
        while right < s_char.len() {
            let right_char = s_char[right];
            right = right + 1;
            if need.contains_key(&right_char) {
                // 预期的字符，添加到窗口统计次数中
                window.entry(right_char).and_modify(|f| { *f = *f + 1 }).or_insert(1);
                // 窗口中包含全部预期字符次数，则窗口中有效字符数+1
                if window[&right_char] == need[&right_char] {
                    valid = valid + 1;
                }
            }
            // 有效字符数等于预期字符数
            while valid >= need.len() {
                // 记录最小子串字符串开始和长度
                if right - left < len {
                    start = left;
                    len = right - left;
                }
                let left_char = s_char[left];
                left = left + 1;
                if need.contains_key(&left_char) {
                    // 窗口左侧字符为预期字符，窗口中字符次数等于预期字符次数，则有效字符减去一，左侧窗口向右移动了一次
                    if window[&left_char] == need[&left_char] {
                        valid = valid - 1;
                    }
                    // 修改窗口中字符次数
                    window.entry(left_char).and_modify(|f| { *f = *f - 1 });
                }
            }
        }
        if len == usize::MAX {
            "".to_string()
        } else {
            s[start..start + len].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string());
    }
}