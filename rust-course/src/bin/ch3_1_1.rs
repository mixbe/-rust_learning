// 函数中的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }

    // 必须返回字符串的所有权
    // let result = String::from("really long time");
    // result.as_str()
}


// 结构体中的生命周期
// 3条生命周期消除规则 a.为每一个参数独自生成一个生命周期 b 函数只有一个引用类型，会把这个参数给输出 c 方法，&self,&mut self


#[derive(Debug)]
struct ImportantException<'a> {
    part: &'a str,
}


// 方法中的生命周期

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


fn main() {
    // let string1 = String::from("long string isn long");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("result: {}", result);
    // }
    // println!("xx");

    // let novel = String::from("Call me Ishmael. Some years ago..");
    // let first_sentence = novel.split('.').next().expect("Cound not find a");
    // let i = ImportantException {
    //     part: first_sentence
    // };
    // println!("i:{:?}", i);



}
