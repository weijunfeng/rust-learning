### rust跨端

## android

1. 安装 cargo-ndk

   ``cargo install cargo-ndk``

2. 使用 cargo-ndk 构建项目

   ``
   cargo ndk -t armv7-linux-androideabi -t aarch64-linux-android -o /Users/wjf/Documents/github/rust-learning/rust-multiplatform/target/jniLibs build --release
   ``

## ios

1. 安装 cargo-lipo

   ``cargo install cargo-lipo``

2. 使用 cargo-lipo 构建项目

   ``cargo lipo --release``

## ohos

1. 安装 ohos

   ``cargo install ohrs``

   - 如安装报错，重新安装 openssl再尝试，mac下命令 `brew install openssl` 

## nodejs