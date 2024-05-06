# log-viewer

**log-viewer: Implementation in Rust of a big log file viewer**

## Features
- [x] Only use rust standard libs
- [x] Viewer log file step by step
- [x] Record search result in file

## Usage log-viewer
Usage: log-viewer [OPTIONS]

Options[interactive-instructions]:
    n,                        next match
    b,                        record content
    d,                        abort search

Options[search-params]:
    -h, --help                 help info
    -num,                      match the number of lines before and after the text content
    -s,                        search text
    -f,                        datasource
    -out,                      the file path for saving search results

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
