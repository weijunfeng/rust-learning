/*
题目描述
有一种简易压缩算法：针对全部由小写英文字母组成的字符串，将其中连续超过两个相同字母的部分压缩为连续个数加该字母，其他部分保持原样不变。

例如：字符串“aaabbccccd”经过压缩成为字符串“3abb4cd”。

请您编写解压函数，根据输入的字符串，判断其是否为合法压缩过的字符串，

若输入合法则输出解压缩后的字符串，否则输出字符串“!error”来报告错误。

输入描述
输入一行，为一个ASCII字符串，长度不会超过100字符，用例保证输出的字符串长度也不会超过100字符。

输出描述
若判断输入为合法的经过压缩后的字符串，则输出压缩前的字符串；

若输入不合法，则输出字符串“!error”。

用例
输入	4dff
输出	ddddff
说明	4d扩展为dddd，故解压后的字符串为ddddff。

输入	2dff
输出	!error
说明	两个d不需要压缩，故输入不合法。

输入	4d@A
输出	!error
说明	全部由小写英文字母组成的字符串压缩后不会出现特殊字符@和大写字母A，故输入不合法。
 */
fn get_result(compression_str: &str) -> String {
    let mut unzip_str = String::new();
    let mut multiple = String::new();
    for char in compression_str.chars() {
        if char >= '0' && char <= '9' {
            multiple.push(char);
            continue;
        }
        if char < 'a' || char > 'z' {
            return "!error".to_string();
        }
        if multiple.is_empty() {
            unzip_str.push(char);
            continue;
        }
        let multiple_num = multiple.parse::<i32>().unwrap();
        if multiple_num <= 2 {
            return "!error".to_string();
        }
        // unzip_str.push_str(&char.to_string().repeat(multiple_num as usize));
        unzip_str.extend(std::iter::repeat(char).take(multiple_num as usize));
        multiple.clear();
    }
    if zip_str(&unzip_str) != compression_str {
        return "!error".to_string();
    }
    return unzip_str;
}

fn zip_str(str: &str) -> String {
    let mut zip_str = String::new();
    let mut repeat = String::new();
    for char in str.chars() {
        if let Some(time_char) = repeat.chars().last() {
            if time_char == char {
                repeat.push(char);
                continue;
            }
            if repeat.len() <= 2 {
                zip_str.push_str(&repeat);
            } else {
                zip_str.push(std::char::from_digit(repeat.len() as u32, 10).unwrap());
                zip_str.push(time_char);
            }
            repeat.clear();
            repeat.push(char);
            continue;
        }
        repeat.push(char);
    }
    zip_str.push_str(&repeat);
    return zip_str;
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test() {
        assert_eq!(super::get_result("4dff"), "ddddff");
        assert_eq!(super::get_result("2dff"), "!error");
        assert_eq!(super::get_result("4d@A"), "!error");
    }
}