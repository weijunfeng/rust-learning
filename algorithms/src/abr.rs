/*
题目描述
数轴×有两个点的序列 A={A1， A2, …, Am}和 B={B1, B2, ..., Bn}， Ai 和 Bj 均为正整数， A、 B 已经从小到大排好序， A、 B 均肯定不为空，

给定一个距离 R（正整数），列出同时满足如下条件的所有（Ai， Bj）数对

条件：

Ai <= Bj
Ai,Bj 距离小于等于 R，但如果 Ai 找不到 R 范围内的 Bj，则列出距它最近的 1 个 Bj，当然此种情况仍然要满足 1，
但如果仍然找不到，就丢弃 Ai。

原型：

车路协同场景，一条路上发生了有很多事件（ A），要通过很多路测设备（ B）广播给路上的车，需要给每个事件找到一个合适的路测设备去发送广播消息。

输入描述
按照人易读的格式输入一行数据，参见输入样例，其中“ ABR={， }”中的每个字符都是关键分割符，输入中无空格，其他均为任意正整数，

输入 A 和 B 已经排好序， A 和 B 的大小不超过 50，正整数范围不会超过 65535。

输出描述z
（ Ai,Bj）数对序列，排列顺序满足序列中前面的 Ax<=后面的 Ay，前面的 Bx<=后面的 By，

因为输入 A 和 B 已经排好序，所以实际上输出结果不用特意排序，排序不是考察点。

用例
输入	A={1,3,5},B={2,4,6},R=1
输出	(1,2)(3,4)(5,6)
说明	无
 */
use std::str::FromStr;

use regex::Regex;

/// 字符串按照指定的分割符分割，并转为 F的数字 集合
fn str_split_i32<F: FromStr>(str: &str, split: &str) -> Vec<F> {
    return str.split(split)
        .filter_map(|str| str.parse().ok()).collect();
}

fn get_result_with_str(str: &str) -> String {
    let regex = Regex::new(r"A=\{(.*)},B=\{(.*)},R=(.+)$");
    if regex.is_err() {
        return String::new();
    }
    if let Some((_, [a, b, r])) = regex.unwrap().captures(str).map(|m| m.extract()) {
        let a: Vec<i32> = str_split_i32(a, ",");
        let b: Vec<i32> = str_split_i32(b, ",");
        if let Ok(r) = r.parse() {
            return get_result(&a[..], &b[..], r);
        }
    }
    return String::new();
}

/// 输出要求 1. Bj>=Ai 2. 若存在Bj-Ai<=R, 则使用，否则使用第一个 Bj>=Ai数
fn get_result(a_array: &[i32], b_array: &[i32], r: i32) -> String {
    let mut result: String = String::new();
    for ai in a_array {
        let mut count = 0;
        for bj in b_array {
            if bj < ai {
                continue;
            }
            if bj - ai <= r || count == 0 {
                result.push_str(&format!("({ai},{bj})"));
                count = count + 1;
            } else {
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::abr::get_result_with_str;

    #[test]
    fn test1() {
        let result = get_result_with_str("A={1,3,5,a},B={2,4,6},R=3");
        assert_eq!("(1,2)(1,4)(3,4)(3,6)(5,6)", result);
    }
}