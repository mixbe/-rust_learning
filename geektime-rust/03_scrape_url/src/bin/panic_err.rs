use std::fs;

// 通过 Result 传递错误
// unwrap() 成功返回结果，失败终止程序，一般简单测试代码使用
// 一般使用 ? 操作符，传播 Result
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = "https://www.rust-lang.org/";
    let output = "url_2_md.md";

    // 获取网页html
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    // html 内容转 md
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    // 保存文件
    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has benn saved in {}", output);
    Ok(())
}