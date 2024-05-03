use std::env;
use std::process;
use log_viewer::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
                                .unwrap_or_else(|err| {
                                    println!("解析参数错误：{err}");
                                    process::exit(1);
                                });

    // println!("搜索文本：{}", config.query);
    // println!("文件路径：{}", config.file_path);


    if let Err(e) = log_viewer::run(&config) {
        println!("程序执行出错：{e}");
        process::exit(1);
    };
}

