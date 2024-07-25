/*
题目描述
某个产品的RESTful API集合部署在服务器集群的多个节点上，近期对客户端访问日志进行了采集，需要统计各个API的访问频次，根据热点信息在服务器节点之间做负载均衡，现在需要实现热点信息统计查询功能。

RESTful API是由多个层级构成，层级之间使用 / 连接，如 /A/B/C/D 这个地址，A属于第一级，B属于第二级，C属于第三级，D属于第四级。

现在负载均衡模块需要知道给定层级上某个名字出现的频次，未出现过用0表示，实现这个功能。

输入描述
第一行为N，表示访问历史日志的条数，0 ＜ N ≤ 100。

接下来N行，每一行为一个RESTful API的URL地址，约束地址中仅包含英文字母和连接符 / ，最大层级为10，每层级字符串最大长度为10。

最后一行为层级L和要查询的关键字。

输出描述
输出给定层级上，关键字出现的频次，使用完全匹配方式（大小写敏感）。

用例
输入	5
/huawei/computing/no/one
/huawei/computing
/huawei
/huawei/cloud/no/one
/huawei/wireless/no/one
2 computing
输出	2
说明	在第二层级上，computing出现了2次，因此输出2
输入	5
/huawei/computing/no/one
/huawei/computing
/huawei
/huawei/cloud/no/one
/huawei/wireless/no/one
4 two
输出	0
说明	存在第四层级的URL上，没有出现two，因此频次是0
 */
use std::collections::HashMap;

fn get_load_statistics(line_count: i32, lines: &[&str], level: usize, value: &str) -> usize {
    let mut level_vec: Vec<HashMap<&str, usize>> = vec![];
    for i in 0..line_count {
        let path_vec: Vec<&str> = lines.get(usize::try_from(i).expect("Conversion failed"))
            .map_or(vec![], |f| f.split("/").collect());
        for (index, path) in path_vec.iter().enumerate() {
            if level_vec.len() <= index {
                level_vec.push(HashMap::new());
            }
            let hashmap = level_vec.get_mut(index).unwrap();
            hashmap.insert(path, hashmap.get(path).map_or(1, |f| f + 1));
        }
    }
    return level_vec.get(level).map_or(0, |f| f.get(value).map_or(0, |f| *f));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let data = vec!["/huawei/computing/no/one", "/huawei/computing", "/huawei", "/huawei/cloud/no/one", "/huawei/wireless/no/one"];
        assert_eq!(2, super::get_load_statistics(5, &data, 2, "computing"));
        assert_eq!(0, super::get_load_statistics(5, &data, 4, "two"));
    }
}