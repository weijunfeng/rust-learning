### rust跨端

## android

1. 安装 android 平台链接库

   ``rustup target add armv7-linux-androideab``

2. 安装 cargo-ndk

   ``cargo install cargo-ndk``

3. 使用 cargo-ndk 构建项目

   ``
   cargo ndk -t armv7-linux-androideabi -t aarch64-linux-android -o /Users/wjf/Documents/github/rust-learning/rust-multiplatform/target/jniLibs build --release
   ``

## ios

1. 安装 android 平台链接库

   ``rustup target add aarch64-apple-ios``

2. 安装 cargo-lipo

   ``cargo install cargo-lipo``

3. 配置项目支持编译静态库输出

   ``[lib] crate-type = ["staticlib", "cdylib"]``

4. 使用 cargo-lipo 构建项目

   ``cargo lipo --release``

## ohos

1. 安装 ohos

   ``cargo install ohrs``

   - 如安装报错，重新安装 openssl再尝试，mac下命令 `brew install openssl` 

2. 导出方法添加 ``#[ohos_node_bindgen]``注解，标识方法可以被 node 端调用

3. node-bindgen 的大致原理如下：
    - FFI（外部函数接口）

      Node.js 的原生模块基于 C++ 和 Node.js 的 N-API（原生API），N-API 提供了一套与 V8 引擎解耦的接口，使原生模块在 Node.js
      版本升级时保持兼容。node-bindgen 底层利用 Rust 的外部函数接口（FFI）能力，通过这些接口与 Node.js 通信。
      Rust 的 FFI 功能允许其调用 C 语言API。因此，node-bindgen 实际上是通过 Rust 的 FFI 调用 Node.js 的 N-API 来创建和管理
      JavaScript 值，以及执行与 JavaScript 环境的交互。
    - 宏和属性

      node-bindgen 提供了一系列宏（例如 #[node_bindgen]），这些宏在编译时自动生成将 Rust 函数暴露为 Node.js
      可调用函数的胶水代码。这个过程包括自动生成用于参数转换和返回值处理的代码，使 Rust 函数能够直接接收来自 JavaScript
      的参数并返回可以直接在 JavaScript 中使用的结果。
    - 内存管理

      Rust 和 JavaScript 之间的内存管理是 node-bindgen 的关键部分。Rust 有自己的内存管理规则，主要基于所有权和生命周期，而
      JavaScript 的内存则由垃圾收集器自动管理。node-bindgen 必须确保在这两种内存管理模型之间正确地桥接，包括处理 Rust
      中的数据所有权转移和确保 JavaScript 对象在需要时保持存活。

    - 异步操作

      Node.js 广泛使用异步操作，而 Rust 也有强大的异步支持。node-bindgen 支持将 Rust 的异步操作暴露给 Node.js。这通过将
      Rust
      的 Future 转换为 Node.js 的 Promise 来实现。node-bindgen 会自动处理这种转换，允许开发者以 Promise 的形式在
      JavaScript
      中接收 Rust 异步操作的结果。

    - 类型转换

      node-bindgen 自动处理 Rust 类型和 JavaScript 类型之间的转换。对于简单类型（如数字和字符串），这通常是直接的。但对于复杂类型（如结构体或枚举），node-bindgen
      生成的代码会负责序列化和反序列化操作，确保两种语言之间可以无缝交换复杂数据结构。

    - 总结

      node-bindgen 利用 Rust 的 FFI 能力、宏系统、强类型系统和异步特性，提供了一种高效、类型安全的方式来将 Rust 代码与
      Node.js 集成。它自动处理大部分繁琐的胶水代码编写工作，使得 Rust 和 Node.js
      之间的交互变得更加简单直接。这样的设计允许开发者专注于实现应用逻辑，而无需深入底层的语言绑定细节。

## nodejs

## rust导出 c

- 在需要导出方法上添加 ``#[no_mangle]`` 宏注解(告诉编译器不要破坏函数名，确保期望的函数名称被编译到产物中)，
  并使用 ``extern "C"`` 标识函数
   ```rust
   #[no_mangle]
   pub extern "C" fn rust_greeting_free(s: *mut c_char) {
       unsafe {
           if s.is_null() { return; }
           CString::from_raw(s)
       };
   }
   ```