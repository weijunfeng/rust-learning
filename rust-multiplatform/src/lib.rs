// use jni::JNIEnv;
// use jni::objects::JClass;
// use jni::sys::jstring;
// use ohos_node_bindgen::derive::ohos_node_bindgen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(feature = "android")]
// #[no_mangle]
// pub extern "system" fn Java_com_haier_uhome_uplus_hook_monitor_app_NativeLib_hello(
//     env: JNIEnv,
//     _class: JClass,
// ) -> jstring {
//     // 将 Rust 字符串转换为 JNI 字符串
//     let result = env.new_string("Hello from Rust!").expect("Couldn't create Java string!");
//     // 返回结果
//     result.into_raw()
// }
// 
// #[cfg(feature = "ohos")]
// #[ohos_node_bindgen]
// pub extern "C" fn add(l: i32, r: i32) -> i32 {
//     l + r
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
