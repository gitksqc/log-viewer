use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::error::Error;
use std::io::{stdin, Write};
// use std::os::windows::fs::FileExt;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)
                    .expect("Should have been able to read the file");

    // if !config.out_path.is_empty() {
    //     fs::File::create(&config.out_path).expect("创建输出文件失败");
    // }
    
    for line in search(&config.query, &contents, &config) {
        // 如果需要输出到文件
        if config.out_path.is_empty() {
            println!("{line}");    
        } else {
            // 指定位置读取文件内容 file.seek_read(buf, offset)
            let mut file = OpenOptions::new().append(true).create(true).open(&config.out_path).expect("创建输出文件失败");
            file.write_all(line.as_bytes()).expect("写入输出文件失败");
            file.write_all("\n".as_bytes()).expect("写入换行符失败");
        }
        
        // 按行输出
        // let uc = user_cmd();
        // if uc.eq("p") {
        //     println!("{line}");
        // } else {
        //     break;
        // }
    }

    Ok(())
}

pub fn search<'a>(qurey: &str, contents: &'a str, config: &Config) -> VecDeque<&'a str> {
    let mut result: VecDeque<&str> = VecDeque::new();
    // 前置的内容 10条
    let mut result_back: VecDeque<&str> = VecDeque::new();
    // 后置的内容 10条
    let mut result_forward: VecDeque<&str> = VecDeque::new();
    // 已经搜索到
    let mut search_flag = false;

    for line in contents.lines() {
        // println!("**** 进入循环 ****");

        if line.contains(qurey) {
            println!("匹配文本： {:?}", line);

            let uc = user_cmd();
            match uc.as_str() {
                "n" => {
                    search_flag = false;
                    // 清理结果集
                    result.drain(..);
                    // 清理匹配文本的后N行
                    result_forward.drain(..);
                    println!("下一次匹配文本");
                    // println!("后续的文本： {:?}", result_forward);
                },
                "b" => {
                    search_flag = true;
                    // 将匹配文本的前N行 及 匹配行 加入结果集
                    result_back.push_back(line);
                    for r in result_back.iter() {
                        result.push_back(r);    
                    }
                    
                    // println!("所有文本： {:?}", result);
                },
                "c" => continue,
                "d" => break,
                &_ => break,
            }
        } else if search_flag {
            if result_forward.len() < config.num {
                result_forward.push_back(line);
                // 将匹配文本的后N行加入结果集
                result.push_back(line);
            } else {
                // 如果后续的文本超过需要的容量，则变更条件，跳出此代码块
                search_flag = false;
            }
            // println!("forward文本数量{:?} 内容： {:?}", result_forward.len(), result_forward);
        } else {
            if result_back.len() >= config.num {
                // 容量满了，把第一条丢弃
                result_back.pop_front();
            }
            result_back.push_back(line);
            // println!("back文本数量{:?}, 内容：{:?}", result_back.len(), result_back);
        }
    }

    result
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub num: usize,
    pub out_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        usage();

        // 文件参数
        let file_arg = String::from("-f");
        // 搜索文本参数
        let search_arg = String::from("-s");
        let num_arg = String::from("-num");
        let out_arg = String::from("-out");

        // 参数
        let mut query = String::new();
        let mut file_path = String::new();
        let mut out_path = String::new();
        let mut num = 10;
        
        // 解析命令行参数
        match args.len() {
            1 => return Err("请指定参数"),
            2 => {
                if args[1].eq("h") || args[1].eq("help") {
                    usage();
                } else {
                    return Err("请指定搜索参数");
                }
            },
            5 | 7 | 9 => {
                if !args.contains(&file_arg) || !args.contains(&search_arg) {
                    return Err("请指定搜索参数 [-f 搜索文件的路径 -s 搜索文本]");
                }

                let mut index = 0;
                for arg in args {
                    // 文件路径
                    if arg.eq(&file_arg) {
                        file_path = args[index + 1].clone();
                    }
                    // 搜索内容
                    if arg.eq(&search_arg) {
                        query = args[index + 1].clone();
                    }
                    // 前后内容行数
                    if arg.eq(&num_arg) {
                        num = args[index + 1].parse::<usize>().unwrap();
                    }
                    // 输出文件
                    if arg.eq(&out_arg) {
                        out_path = args[index + 1].clone();
                    }

                    index += 1;
                }
            },
            
            0_usize | 3_usize.. => {return Err("请指定合理的搜索参数");},
        }
               
        println!("参数：：： {query}, {file_path}, {num}, {:?}", out_path);
        Ok(Config {query, file_path, num, out_path})
    }
}

pub fn user_cmd() -> String {
    let mut cmd = String::new();
    stdin()
        .read_line(&mut cmd)
        .expect("读取用户指令失败");

    cmd.trim().to_lowercase()
}

fn usage() {
    // -V, --version              Print version
    // --flag-b               first flag
    // --option-b <OPTION_B>  first option
    // --flag-a               second flag
    // --option-a <OPTION_A>  second option
    let usage = r#"
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
"#;
    println!("{usage}");
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let q = "duct";
//         let c = "\
// Rust:
// safe, fast, productive.
// Pick three.
// ";

//         assert_eq!(vec!["safe, fast, productive."], search(q, c));
//     }
// }