/*
题目描述
给定一个连续不包含空格的字符串，该字符串仅包含英文小写字母及英文标点符号（逗号、分号、句号），同时给定词库，对该字符串进行精确分词。

说明：

精确分词：字符串分词后，不会出现重叠。即"ilovechina"，不同词库可分割为"i,love,china"，"ilove,china"，不能分割出现重叠的"i,ilove,china"，i 出现重叠
标点符号不成词，仅用于断句
词库：根据外部知识库统计出来的常用词汇例：dictionary = ["i", "love", "china", "lovechina", "ilove"]
分词原则：采用分词顺序优先且最长匹配原则

"ilovechina"，假设分词结果 [i,ilove,lo,love,ch,china,lovechina]，则输出 [ilove,china]

错误输出：[i,lovechina]，原因："ilove" > 优先于 "lovechina" 成词

错误输出：[i,love,china]，原因："ilove" > "i"遵循最长匹配原则
输入描述
第一行输入待分词语句 "ilovechina"

字符串长度限制：0 < length < 256
第二行输入中文词库 "i,love,china,ch,na,ve,lo,this,is,this,word"

词库长度限制：1 < length < 100000
输出描述
按顺序输出分词结果 "i,love,china"

用例
输入	ilovechina
i,love,china,ch,na,ve,lo,this,is,the,word
输出	i,love,china
说明	无
输入	iat
i,love,china,ch,na,ve,lo,this,is,the,word,beauti,tiful,ful
输出	i,a,t
说明	
单个字母，

不在词库中且不成词则输出单个字母

输入	ilovechina,thewordisbeautiful
i,love,china,ch,na,ve,lo,this,is,the,word,beauti,tiful,ful
输出	i,love,china the,word,is,beauti,ful
说明	标点符号为英文标点符号
 */

/// 1. 对字典按照长度排序后倒序，为了匹配到最长的一个字段数据
/// 2. 遍历句子，拿第一个字母从排序后的词典中过滤出最长的单词，找到后继续遍历剩余部分，未找到则添加该单词到结果中，注意处理逗号分割符
fn get_segmentation(word: &str, dictionary: &str) -> String {
    let mut dictionarys = dictionary.split(',').collect::<Vec<&str>>();
    dictionarys.sort_by_key(|f| f.len());
    dictionarys.reverse();
    println!("dictionarys:{dictionarys:?}");
    let mut result = String::new();
    let mut index = 0;
    while index < word.len() {
        let index_str = &word[index..];
        let mut found_dictionary = &index_str[0..1];
        if found_dictionary != "," {
            for dictionary in dictionarys.iter() {
                if dictionary.starts_with(found_dictionary) && index_str.starts_with(dictionary) {
                    found_dictionary = *dictionary;
                    break;
                }
            }
            if !result.is_empty() && !result.ends_with(",") {
                result.push(',');
            }
        }
        result.push_str(found_dictionary);
        index = index + found_dictionary.len();
    }
    result
}

#[cfg(test)]
pub mod tests {
    use crate::word_segmentation::get_segmentation;

    #[test]
    fn test() {
        assert_eq!("i,love,china", get_segmentation("ilovechina",
                                                    "i,love,china,ch,na,ve,lo,this,is,the,word"));
        assert_eq!("i,a,t", get_segmentation("iat",
                                             "i,love,china,ch,na,ve,lo,this,is,the,word,beauti,tiful,ful"));
        assert_eq!("i,love,china,the,word,is,beauti,ful", get_segmentation("ilovechina,thewordisbeautiful",
                                                                           "i,love,china,ch,na,ve,lo,this,is,the,word,beauti,tiful,ful"));
    }
}