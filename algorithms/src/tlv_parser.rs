/*
两端通过TLV格式的报文来通信，现在收到对端的一个TLV格式的消息包，要求生成匹配后的(tag, length, valueOffset)列表。

具体要求如下:

(1)消息包中多组tag、length、value紧密排列，其中tag,length各占1字节(uint8_t) , value所占字节数等于length的值

(2)结果数组中tag值已知，需要填充每个tag对应数据的length和valueOffset值(valueOffset为value在原消息包中的起始偏移量（从0开始，以字节为单位))，

即将消息包中的tag与结果数组中的tag进行匹配（可能存在匹配失败的情况，若结果数组中的tag在消息包中找不到，则length和valueOffset都为0)

(3)消息包和结果数组中的tag值都按升序排列，且不重复

(4)此消息包未被篡改，但尾部可能不完整，不完整的一组TLV请丢弃掉

输入描述
第一行: 一个字符串，代表收到的消息包。字符串长度在10000以内。

说明1: 字符串使用十六进制文本格式（字母为大写）来展示消息包的数据，如0F04ABABABAB代表一组TLV:前两个字符(0F）代表tag值为15，接下来两个字符（04）代表length值为4字节，接下来8个字符即为4字节的value。
说明2: 输入字符串中，每一组TLV紧密排列，中间无空格等分隔符
第二行: 需要匹配的tag数量n (0 < n <1000) 。

后面n行: 需要匹配的n个tag值（十进制表示)，递增排列。

输出描述
和需要匹配的n个tag对应的n行匹配结果，每一行由长度和偏移量组成
 */
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct TLV {
    tag: i32,
    length: i32,
    offset: i32,
}


fn hex_str_to_byte_array(message: &str) -> Vec<u8> {
    if message.len() % 2 != 0 {
        return vec![];
    }
    return (0..message.len()).step_by(2)
        .map(|i| {
            u8::from_str_radix(&message[i..i + 2], 16).unwrap()
        }).collect();
}

fn tlv_parser(messages: Vec<u8>) -> HashMap<i32, TLV> {
    let mut i = 0;
    let mut tlv_map = HashMap::new();
    while i < messages.len() {
        if i + 2 > messages.len() {
            return HashMap::default();
        }
        let tag = messages[i] as i32;
        let length = messages[i + 1] as i32;
        let offset = i + 2;
        tlv_map.insert(tag, TLV {
            tag,
            length,
            offset: offset as i32,
        });
        i = offset + (length as usize);
        if i > messages.len() {
            return HashMap::default();
        }
    }
    return tlv_map;
}

fn message_parser(message: &str, tags: Vec<i32>) -> Vec<TLV> {
    if message.len() > 10000 {
        return vec![];
    }
    let messages = hex_str_to_byte_array(message);
    let tlv_map = tlv_parser(messages);
    let mut tlv_result = Vec::new();
    for tag in tags {
        let tlv = tlv_map.get(&tag);
        match tlv {
            None => {
                tlv_result.push(TLV {
                    tag,
                    length: 0,
                    offset: 0,
                });
            }
            Some(tlv) => {
                tlv_result.push(*tlv);
            }
        }
    }
    tlv_result
}


#[cfg(test)]
pub mod tests {
    use crate::tlv_parser::message_parser;

    #[test]
    fn test() {
        let result = message_parser("0F04ABABABAB", vec![15]);
        assert_eq!("[TLV { tag: 15, length: 4, offset: 2 }]", format!("{:?}", result));
        let result = message_parser("0F04ABABABAB1001FF", vec![15, 17]);
        assert_eq!("[TLV { tag: 15, length: 4, offset: 2 }, TLV { tag: 17, length: 0, offset: 0 }]", format!("{:?}", result));
    }
}