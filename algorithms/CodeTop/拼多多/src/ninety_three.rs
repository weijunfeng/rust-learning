/*
https://leetcode.cn/problems/restore-ip-addresses/description/
93. 复原 IP 地址
有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。

例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。

 

示例 1：

输入：s = "25525511135"
输出：["255.255.11.135","255.255.111.35"]
示例 2：

输入：s = "0000"
输出：["0.0.0.0"]
示例 3：

输入：s = "101023"
输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 

提示：

1 <= s.length <= 20
s 仅由数字组成
 */
struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();
        let mut result: Vec<String> = Vec::new();
        Solution::back_track(&s, 0, &mut paths, &mut result);
        result
    }
    
    fn back_track(s: &str, start: usize, paths: &mut Vec<String>, result: &mut Vec<String>) {
        if paths.len() == 4 {
            if start == s.len() {
                result.push(paths.join("."));
            }
            return;
        }
        for i in 1..=3 {
            if start + i > s.len() {
                break;
            }
            let path = &s[start..start + i];
            if Solution::is_valid_path(path) {
                paths.push(path.to_string());
                Solution::back_track(s, start + i, paths, result);
                paths.remove(paths.len() - 1);
            }
        }
    }

    fn is_valid_path(path: &str) -> bool {
        if path.len() > 1 && path.starts_with("0") {
            return false;
        }
        let num = path.parse::<i32>().unwrap();
        return num >= 0 && num <= 255;
    }
}