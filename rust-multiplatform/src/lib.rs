use std::ffi::{c_char, CStr, CString};

#[cfg(feature = "android")]
use jni::{JNIEnv, objects::JClass, sys::jstring};
#[cfg(feature = "ohos")]
use ohos_node_bindgen::derive::ohos_node_bindgen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(feature = "android")]
#[no_mangle]
pub extern "system" fn Java_com_haier_uhome_uplus_hook_monitor_app_NativeLib_hello(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    // 将 Rust 字符串转换为 JNI 字符串
    let result = env.new_string("Hello from Rust!").expect("Couldn't create Java string!");
    // 返回结果
    result.into_raw()
}

//no_mangle 来告诉编译器不要破坏函数名，确保我们的函数名称被导入到 C 文件
#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = c_str.to_str().unwrap_or_else(|_| "there");

    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[cfg(feature = "ohos")]
#[ohos_node_bindgen]
#[no_mangle]
pub extern "C" fn add(l: i32, r: i32) -> i32 {
    l + r
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
