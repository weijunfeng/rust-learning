/*
题目描述
华为商城举办了一个促销活动，如果某顾客是某一秒内最早时刻下单的顾客（可能是多个人），则可以获取免单。

请你编程计算有多少顾客可以获取免单。

输入描述
输入为 n 行数据，每一行表示一位顾客的下单时间

以（年-月-日时-分-秒.毫秒） yyyy-MM-ddHH:mm:ss.fff 形式给出。

0<n<50000
2000<yyyy<2020
0<MM<=12
0<dd<=28
0<=HH<=23
0<=mm<=59
0<=ss<=59
0<=fff<=999
所有输入保证合法。

输出描述
输出一个整数，表示有多少顾客可以获取免单。

用例
输入	3
2019-01-01 00:00:00.001
2019-01-01 00:00:00.002
2019-01-01 00:00:00.003
输出	1
说明	样例 1 中，三个订单都是同一秒内下单，只有第一个订单最早下单，可以免单。
输入	3
2019-01-01 08:59:00.123
2019-01-01 08:59:00.123
2018-12-28 10:08:00.999
输出	3
说明	样例 2 中，前两个订单是同一秒内同一时刻（也是最早）下单，都可免单，第三个订单是当前秒内唯一一个订单（也是最早），也可免单。
输入	5
2019-01-01 00:00:00.004
2019-01-01 00:00:00.004
2019-01-01 00:00:01.006
2019-01-01 00:00:01.006
2019-01-01 00:00:01.005
输出	3
说明	样例 3 中，前两个订单是同一秒内同一时刻（也是最早）下单，第三第四个订单不是当前秒内最早下单，不可免单，第五个订单可以免单。
 */
use std::collections::HashMap;

struct Free {
    millisecond: i32,
    count: i32,
}

impl Free {
    fn new(millisecond: i32) -> Self {
        return Free { millisecond, count: 1 };
    }

    fn count_increment(&mut self) {
        self.count = self.count + 1
    }
}

fn free_count(orders: Vec<String>) -> i32 {
    let mut order_map: HashMap<String, Free> = HashMap::new();
    for order in orders {
        let millisecond_index = order.len() - 3;
        let time = &order[0..millisecond_index];
        let millisecond: i32 = *&order[millisecond_index..order.len()].parse::<i32>().unwrap();
        if let Some(time_vec) = order_map.get_mut(time) {
            if time_vec.millisecond > millisecond {
                *time_vec = Free::new(millisecond)
            } else if time_vec.millisecond == millisecond {
                time_vec.count_increment()
            }
        } else {
            order_map.insert(time.to_string(), Free::new(millisecond));
        }
    }
    let mut count = 0;
    for order in order_map {
        count = count + order.1.count
    }
    count
}

#[cfg(test)]
pub mod tests {
    use crate::free_count::free_count;

    #[test]
    fn test() {
        let count = free_count(vec!["2019-01-01 00:00:00.001".to_string(),
                                    "2019-01-01 00:00:00.002".to_string(),
                                    "2019-01-01 00:00:00.003".to_string()]);
        assert_eq!(1, count);
        let count = free_count(vec!["2019-01-01 08:59:00.123".to_string(),
                                    "2019-01-01 08:59:00.123".to_string(),
                                    "2018-12-28 10:08:00.999".to_string()]);
        assert_eq!(3, count);
        let count = free_count(vec!["2019-01-01 00:00:00.004".to_string(),
                                    "2019-01-01 00:00:00.004".to_string(),
                                    "2019-01-01 00:00:01.006".to_string(),
                                    "2019-01-01 00:00:01.006".to_string(),
                                    "2019-01-01 00:00:01.005".to_string(),
        ]);
        assert_eq!(3, count);
    }
}