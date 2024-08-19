/*
https://leetcode.cn/problems/reverse-words-in-a-string/description/
151. 反转字符串中的单词
给你一个字符串 s ，请你反转字符串中 单词 的顺序。

单词 是由非空格字符组成的字符串。s 中使用至少一个空格将字符串中的 单词 分隔开。

返回 单词 顺序颠倒且 单词 之间用单个空格连接的结果字符串。

注意：输入字符串 s中可能会存在前导空格、尾随空格或者单词间的多个空格。返回的结果字符串中，单词间应当仅用单个空格分隔，且不包含任何额外的空格。

 

示例 1：

输入：s = "the sky is blue"
输出："blue is sky the"
示例 2：

输入：s = "  hello world  "
输出："world hello"
解释：反转后的字符串中不能存在前导空格和尾随空格。
示例 3：

输入：s = "a good   example"
输出："example good a"
解释：如果两个单词间有多余的空格，反转后的字符串需要将单词间的空格减少到仅有一个。
 

提示：

1 <= s.length <= 104
s 包含英文大小写字母、数字和空格 ' '
s 中 至少存在一个 单词
 

进阶：如果字符串在你使用的编程语言中是一种可变数据类型，请尝试使用 O(1) 额外空间复杂度的 原地 解法。
*/
struct Solution;

impl Solution {
    // 三步，第一步 反转字符串字符，第二步 反转字符串中单个单词，第三步 删除多余空格
    pub fn reverse_words(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut chars: Vec<char> = s.chars().collect();
        let size = chars.len();
        Self::reverse(&mut chars, 0, size - 1);
        Self::word_reverse(&mut chars);
        let new_len = Self::clear_space(&mut chars);
        chars.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("").get(..new_len).unwrap().to_string()
    }

    pub fn clear_space(chars: &mut Vec<char>) -> usize {
        let mut i = 0;
        let mut j = 0;
        let size = chars.len();
        while j < size {
            // 跳过单词前面空格
            while j < size && chars[j] == ' ' {
                j = j + 1;
            }
            // 移动单词位置
            while j < size && chars[j] != ' ' {
                chars[i] = chars[j];
                i = i + 1;
                j = j + 1;
            }
            // 跳过单词后面空格
            while j < size && chars[j] == ' ' {
                j = j + 1;
            }
            // 单词添加空格
            if j < size {
                chars[i] = ' ';
                i = i + 1;
            }
        }
        i
    }

    pub fn word_reverse(chars: &mut Vec<char>) {
        let mut i = 0;
        let mut j = 0;
        let size = chars.len();
        while j < size {
            while i < size && chars[i] == ' ' {
                i = i + 1;
            }
            j = i;
            while j < size && chars[j] != ' ' {
                j = j + 1;
            }
            Self::reverse(chars, i, j - 1);
            i = j;
        }
    }

    pub fn reverse(chars: &mut Vec<char>, start: usize, end: usize) {
        let mut left = start;
        let mut right = end;
        while left < right {
            let temp = chars[left];
            chars[left] = chars[right];
            chars[right] = temp;
            left = left + 1;
            right = right - 1;
        }
    }
}