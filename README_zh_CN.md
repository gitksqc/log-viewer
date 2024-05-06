# log-viewer

**log-viewer: 基于Rust实现，大日志文件查看的小工具**

## Features
- [x] 仅使用rust标准库
- [x] 可以按需查看日志内容
- [x] 记录日志内容的搜索结果

## Usage log-viewer
Usage: log-viewer [OPTIONS]

Options[交互指令]:
    n,                        下一次匹配
    b,                        记录当前匹配内容
    d,                        结束匹配

Options[查询参数]:
    -h, --help                 帮助信息
    -num,                      匹配文本前后内容的行数
    -s,                        搜索文本
    -f,                        搜索数据源
    -out,                      搜索结果保存的文件路径

Cargo.toml
```toml
[dependencies]

```

main.rs
```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
                                .unwrap_or_else(|err| {
                                    println!("解析参数错误：{err}");
                                    process::exit(1);
                                });

    if let Err(e) = log_viewer::run(&config) {
        println!("程序执行出错：{e}");
        process::exit(1);
    };
}
```


## log-viewer CLI
### Install Rust for Linux/MacOS
```bash 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Install Rust for Windows
```bash
https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
```
### Build executable log-viewer
```bash
git clone https://github.com/gitksqc/log-viewer.git
cd log-viewer
cargo build --release
```
### Run executable log-viewer
```bash
cd ./target/release/
.\target\release\log-viewer.exe -s "mes返回的数据" -f sample.log -num 5 -out out666.log
```
