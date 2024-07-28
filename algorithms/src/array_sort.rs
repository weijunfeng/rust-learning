/*
题目描述
 2012伦敦奥运会即将到来，大家都非常关注奖牌榜的情况，现在我们假设奖牌榜的排名规则如下:

首先gold medal数量多的排在前面
其次silver medal数量多的排在前面
然后bronze medal数量多的排在前面
若以上三个条件仍无法区分名次，则以国家名称的字典顺序排定。
我们假设国家名称不超过二十个字符，各类奖牌数不超过100，且大于0.

输入描述
第一行输入一个整数N（0<N<21），代表国家数量，

然后接下来的N行，每行包含：

一个字符串Name表示各个国家的名称和三个整数Gi,Si,Bi表示每个获得的gold medal,silver medal,bronze medal的数量，以空格隔开，如(China 51 20 21),

具体见样例输入。

5
China 32 28 34
England 12 34 22
France 23 33 2
Japan 12 34 25
Rusia 23 43 0

输出描述
输出奖牌榜的依次顺序，只输出国家名称，各占一行，具体见样例输出。

China
Rusia
France
Japan
England

用例
输入	5
China 32 28 34
England 12 34 22
France 23 33 2
Japan 12 34 25
Rusia 23 43 0
输出	China
Rusia
France
Japan
England
说明	无
 */
use itertools::Itertools;

#[derive(Debug)]
struct Smedal<'a> {
    country: &'a str,
    gold: i32,
    silver: i32,
    bronze: i32,
}

fn get_smedal_table(smedal_table: Vec<&str>) -> Vec<&str> {
    smedal_table.iter().map(|smedal| {
        let mut split = smedal.split_whitespace();
        let country = split.next().unwrap();
        let gold = split.next().unwrap().parse::<i32>().unwrap();
        let silver = split.next().unwrap().parse::<i32>().unwrap();
        let bronze = split.next().unwrap().parse::<i32>().unwrap();
        Smedal {
            country,
            gold,
            silver,
            bronze,
        }
    })
        .sorted_by(|f1, f2| {
            f1.gold.cmp(&f2.gold)
                .then_with(|| f1.silver.cmp(&f2.silver))
                .then_with(|| f1.bronze.cmp(&f2.bronze))
                .then_with(|| f1.country.cmp(&f2.country))
                .reverse()
        })
        .map(|f| f.country)
        .collect::<Vec<&str>>()
}

#[cfg(test)]
pub mod tests {
    use crate::array_sort::get_smedal_table;

    #[test]
    fn test() {
        let result = get_smedal_table(vec!["China 32 28 34",
                                           "England 12 34 22",
                                           "France 23 33 2",
                                           "Japan 12 34 25",
                                           "Rusia 23 43 0"]);
        println!("{result:?}");
    }
}