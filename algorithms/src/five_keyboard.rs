/*
题目描述
有一个特殊的5键键盘，上面有a，ctrl-c，ctrl-x，ctrl-v，ctrl-a五个键。

a键在屏幕上输出一个字母a；

ctrl-c将当前选择的字母复制到剪贴板；

ctrl-x将当前选择的字母复制到剪贴板，并清空选择的字母；

ctrl-v将当前剪贴板里的字母输出到屏幕；

ctrl-a选择当前屏幕上的所有字母。

注意：

剪贴板初始为空，新的内容被复制到剪贴板时会覆盖原来的内容
当屏幕上没有字母时，ctrl-a无效
当没有选择字母时，ctrl-c和ctrl-x无效
当有字母被选择时，a和ctrl-v这两个有输出功能的键会先清空选择的字母，再进行输出
给定一系列键盘输入，输出最终屏幕上字母的数量。

输入描述
输入为一行，为简化解析，用数字1 2 3 4 5代表a，ctrl-c，ctrl-x，ctrl-v，ctrl-a五个键的输入，数字用空格分隔。
输出描述
输出一个数字，为最终屏幕上字母的数量。
用例
输入	1 1 1
输出	3
说明	连续键入3个a，故屏幕上字母的长度为3。
输入	1 1 5 1 5 2 4 4
输出	2
说明
输入两个a后ctrl-a选择这两个a，再输入a时选择的两个a先被清空，所以此时屏幕只有一个a，
后续的ctrl-a，ctrl-c选择并复制了这一个a，最后两个ctrl-v在屏幕上输出两个a，
故屏幕上字母的长度为2（第一个ctrl-v清空了屏幕上的那个a）。
*/
fn get_result_command(commands: &str) -> usize {
    let command_vec: Vec<i32> = commands
        .split(" ")
        .map(|command| command.parse())
        .filter(|command| command.is_ok())
        .map(|command| command.unwrap())
        .collect();
    return get_screen_count(&command_vec[..]);
}

/// 思路
/// 核心有两个缓存数据的集合，一个缓存当前屏幕数据，一个缓存剪切板数据
/// 两个数据清空时机：前置条件选中全部数据，1. 当前屏幕在输入数据、剪切数据、粘贴数据时清空，2. 剪切板在 复制数据、剪切数据时清空
/// 全选数据状态清除时机与屏幕数据清空保持一致，即屏幕有数据才能保持全选状态
fn get_screen_count(commands: &[i32]) -> usize {
    let mut screen: Vec<&str> = vec![];
    let mut clip: Vec<&str> = vec![];
    let mut is_select_all = false;
    for command in commands {
        match command {
            1 => {
                // a
                if is_select_all {
                    screen.clear();
                    is_select_all = false;
                }
                screen.push("a");
            }
            2 => {
                // ctrl-c
                if is_select_all {
                    clip.clear();
                    clip.extend_from_slice(&screen[..]);
                }
            }
            3 => {
                // ctrl-x
                if is_select_all {
                    clip.clear();
                    clip.extend_from_slice(&screen[..]);
                    screen.clear();
                    is_select_all = false;
                }
            }
            4 => {
                //ctrl-v
                if is_select_all {
                    screen.clear();
                    is_select_all = false;
                }
                screen.extend_from_slice(&clip[..]);
            }
            5 => {
                //ctrl-a
                is_select_all = !screen.is_empty();
            }
            _ => {}
        }
    }
    screen.len()
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        assert_eq!(3, super::get_result_command("1 1 1"));
        assert_eq!(2, super::get_result_command("1 1 5 1 5 2 4 4"))
    }
}
