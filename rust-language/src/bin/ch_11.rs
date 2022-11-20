// 编写自动化测试

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    pub fn new(calculation: T, value: Option<u32>) -> Self {
        Self { calculation, value }
    }
}

fn main() {
    // 函数和闭包的定义语法
    // fn add_one_v1(x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1;

    // 所有的闭包至少实现以下trait之一 fn  fnmut fnonce

    println!("Starting");
}