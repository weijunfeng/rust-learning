/*
题目描述
给定一个只包含大写英文字母的字符串S，要求你给出对S重新排列的所有不相同的排列数。

如：S为ABA，则不同的排列有ABA、AAB、BAA三种。

输入描述
输入一个长度不超过10的字符串S，我们确保都是大写的。

输出描述
输出S重新排列的所有不相同的排列数（包含自己本身）。

输入	ABA
输出	3
说明	无
输入	ABCDEFGHHA
输出	907200
说明	无
 */
use std::collections::HashMap;

fn factorial(n: i32) -> i32 {
    let mut result = 1;
    let mut i = 1;
    while i <= n {
        result = result * i;
        i = i + 1;
    }
    result
}

// total/ a(count)!/b(count)!
fn count_unique_permutations(string: &str) -> i32 {
    let mut char_count = HashMap::<char, i32>::new();
    for char in string.chars() {
        let count = char_count.get_mut(&char);
        match count {
            None => {
                char_count.insert(char, 1);
            }
            Some(count) => {
                *count = *count + 1;
            }
        }
    }
    let mut total = factorial(string.len() as i32);
    for (_, count) in char_count {
        if count > 1 {
            total = total / factorial(count);
        }
    }
    total
}

#[cfg(test)]
pub mod tests {
    use crate::unique_permutations::count_unique_permutations;

    #[test]
    fn test() {
        assert_eq!(3, count_unique_permutations("ABA"));
        assert_eq!(907200, count_unique_permutations("ABCDEFGHHA"));
    }
}