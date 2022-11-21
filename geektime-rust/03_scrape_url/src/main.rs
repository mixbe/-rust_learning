use std::fs;

/// 获取url 链接内容，并转成 markdown
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "url_2_md.md";

    // 获取网页html
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    // html 内容转 md
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    // 保存文件
    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has benn saved in {}", output);
}
