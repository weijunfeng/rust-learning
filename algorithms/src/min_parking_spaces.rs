/*
题目描述
特定大小的停车场，数组cars[]表示，其中1表示有车，0表示没车。

车辆大小不一，小车占一个车位（长度1），货车占两个车位（长度2），卡车占三个车位（长度3）。

统计停车场最少可以停多少辆车，返回具体的数目。

输入描述
整型字符串数组cars[]，其中1表示有车，0表示没车，数组长度小于1000。

输出描述
整型数字字符串，表示最少停车数目。

用例
输入	1,0,1
输出	2
说明
1个小车占第1个车位

第二个车位空

1个小车占第3个车位

最少有两辆车

输入	1,1,0,0,1,1,1,0,1
输出	3
说明
1个货车占第1、2个车位

第3、4个车位空

1个卡车占第5、6、7个车位

第8个车位空

1个小车占第9个车位

最少3辆车
 */

fn min_parking_spaces(spaces: String) -> usize {
    let spaces = spaces.replace(",", "")
        .replace("111", "x")
        .replace("11", "x")
        .replace("1", "x");
    spaces.chars().filter(|c| *c == 'x').count()
}

#[cfg(test)]
pub mod tests{
    use crate::min_parking_spaces::min_parking_spaces;

    #[test]
    fn test(){
        assert_eq!(3 , min_parking_spaces("1,1,0,0,1,1,1,0,1".to_string()));
        assert_eq!(2 , min_parking_spaces("1,0,1".to_string()));
    }
}