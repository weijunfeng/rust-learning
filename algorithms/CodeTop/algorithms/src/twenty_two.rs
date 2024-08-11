/*
https://leetcode.cn/problems/generate-parentheses/description/
22. 括号生成
数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

 

示例 1：

输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
示例 2：

输入：n = 1
输出：["()"]
 

提示：

1 <= n <= 8
 */
struct Solution;
impl Solution {
    // 采用回溯法，回溯法：先加入当前值，递归后移除当前值
    // 递归深度为“(”数和“)”都等于 n
    // 递归条件：
    // 	•	如果当前的左括号数量小于 n，我们可以添加一个左括号 '('。
    // 	•	如果当前的右括号数量小于左括号数量，意味着我们可以添加一个右括号 ')'。
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Solution::dfs(n, 0, 0, &mut result, &mut String::new());
        result
    }

    pub fn dfs(n: i32, left: i32, right: i32, result: &mut Vec<String>, path: &mut String) {
        if left == n && right == n {
            result.push(path.clone());
        }
        if left < n {
            path.push_str("(");
            Solution::dfs(n, left + 1, right, result, path);
            path.pop();
        }
        // 如果当前的右括号数量小于左括号数量，意味着我们可以添加一个右括号 ')'
        if right + 1 <= left {
            path.push_str(")");
            Solution::dfs(n, left, right + 1, result, path);
            path.pop();
        }
    }
}