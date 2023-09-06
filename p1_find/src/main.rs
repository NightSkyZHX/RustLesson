use regex::Regex;
use std::{env, process};
fn main() {

    let args: Vec<String> = env::args().collect();
    // 参数1：搜索目录；参数2：要搜索的正则表达式。
    //思考一下：如果用户输入的参数太多，应该怎么样？
    //参数太多，就循环处理，每两个参数为一组，第一个参数为搜索目录，第二个参数为正则表达式
    if args.len() < 3 {
        eprintln!("使用方式：{} <目标目录＞ <要搜索的正则表达式＞", args[0]);
        process::exit(1);
    }
    
    
    for x in 1..=(args.len()-1) /2 {
        let pattern = &args[2*x];
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(err) => {
                eprintln!("无效的正则表达式'{}':{}", pattern, err);
                process::exit(1);
            }
        };
        match find_mod::find(&args[2*x-1], &regex) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!("未找到匹配项。");
                } else {
                    println!("找到以下匹配项：");
                    for file in matches {
                        println!("{}", file);
                    }
                }
                println!();
            }
            Err(error) => {
                eprintln!("发生错误：{}", error);
                process::exit(1);
            }
        }
    }
   
}


pub mod find_mod {
    use regex::Regex;
    use std::{fs, path::Path};

    pub fn find<P: AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut matches = Vec::new();
        walk_tree(root.as_ref(), regex, &mut matches)?;
        Ok(matches)
    }
    
    fn walk_tree(
        dir: &Path,
        regex: &Regex,
        matches: &mut Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在搜索：{}", dir.display());
        //如果不是，应该怎么办呢？
        //直接匹配文件名
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                    walk_tree(&path, regex, matches)?;
            }
        } else if let Some(filename) = dir.file_name().and_then(|s| s.to_str()) {
            if regex.is_match(filename) {
                matches.push(dir.to_string_lossy().to_string());
            }
        }
        Ok(())
    }
    
}
