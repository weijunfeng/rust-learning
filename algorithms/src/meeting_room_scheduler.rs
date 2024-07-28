/*
现有若干个会议，所有会议共享一个会议室，用数组表示各个会议的开始时间和结束时间，格式为：

[[会议1开始时间, 会议1结束时间], [会议2开始时间, 会议2结束时间]]

请计算会议室占用时间段。

输入描述
第一行输入一个整数 n，表示会议数量

之后输入n行，每行两个整数，以空格分隔，分别表示会议开始时间，会议结束时间

输出描述
输出多行，每个两个整数，以空格分隔，分别表示会议室占用时间段开始和结束

备注
会议室个数范围：[1, 100]
会议室时间段：[1, 24]
 */
use std::cmp::max;

fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut sort_intervals = intervals.to_vec();
    if intervals.len() <= 1 {
        return sort_intervals;
    }
    sort_intervals.sort_by_key(|f| f.0);
    let mut curr_data = sort_intervals[0];
    let mut merge_result = Vec::new();
    for &(start, end) in sort_intervals.iter().skip(1) {
        if curr_data.1 >= start {
            curr_data.1 = max(curr_data.1, end)
        } else {
            merge_result.push(curr_data);
            curr_data = (start, end);
        }
    }
    merge_result.push(curr_data);
    return merge_result;
}

#[cfg(test)]
mod tests {
    use crate::meeting_room_scheduler::merge_intervals;

    #[test]
    fn test() {
        assert_eq!(vec![(1, 5)], merge_intervals(&vec![(1, 4), (4, 5)]));
        assert_eq!(vec![(1, 5), (7, 9), (14, 18)], merge_intervals(&vec![(1, 4), (2, 5), (7, 9), (14, 18)]));
    }
}