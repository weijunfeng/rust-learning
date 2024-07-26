/*
两数之和绝对值最小（Java & JS & Python）
题目描述
给定一个从小到大的有序整数序列（存在正整数和负整数）数组 nums ，请你在该数组中找出两个数，其和的绝对值(|nums[x]+nums[y]|)为最小值，并返回这个绝对值。
每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

输入描述
一个通过空格分割的有序整数序列字符串，最多1000个整数，且整数数值范围是 -65535~65535。

输出描述
两数之和绝对值最小值

用例
输入	-3 -1 5 7 11 15
输出	2
说明	因为 |nums[0] + nums[2]| = |-3 + 5| = 2 最小，所以返回 2。

 */

fn find_two_sum_closest(data: &[i32]) -> Option<(i32, i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut sort_data = data.to_vec();
    sort_data.sort();
    let mut left = 0;
    let mut right = sort_data.len() - 1;
    let mut min_abs_sum = i32::MAX;
    let mut data = (0, 0);
    while left < right {
        let left_value = sort_data.get(left).unwrap();
        let right_value = sort_data.get(right).unwrap();
        let curr_sum = left_value + right_value;
        let curr_sum_abs = curr_sum.abs();
        if curr_sum_abs <= min_abs_sum {
            min_abs_sum = curr_sum_abs;
            data = (*left_value, *right_value);
        }
        if curr_sum > 0 {
            right = right - 1;
        } else if curr_sum < 0 {
            left = left + 1;
        } else {
            break;
        }
    }
    Some(data)
}

#[cfg(test)]
mod tests {
    use crate::two_sum_closest::find_two_sum_closest;

    #[test]
    fn test() {
        let data = [-3, -1, 5, 7, 11, 15];
        let result = find_two_sum_closest(&data);
        assert_eq!(Some((-3, 5)), result);
    }
}