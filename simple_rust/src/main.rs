// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     pub fn new(x: T, y: U) -> Self {
//         Self { x, y }
//     }
//     pub fn x(&self) -> &T {
//         &self.x
//     }
//     pub fn y(&self) -> &U {
//         &self.y
//     }
//
//     // 混合 泛型用于参数
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// impl Point<f32, f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
//
// fn largest_int(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
//
// // copy约束
// fn largest<T>(v: &[T]) -> &T
//     where T: PartialOrd + Copy {
//     let mut largest = v[0];
//     for &value in v.iter() {
//         if value > largest {
//             largest = value;
//         }
//     }
//     largest
// }
//
// // 引用
// fn largest_v1<T>(v: &[T]) -> &T
//     where T: PartialOrd {
//     let mut largest = &v[0];
//     for value in v.iter() {
//         if value > largest {
//             largest = value;
//         }
//     }
//     largest
// }
//
// // Clone  效率不是很高
// fn largest_v2<T>(v: &[T]) -> &T
//     where T: PartialOrd + Clone {
//     let mut largest = v[0].clone();
//     for &value in v.iter() {
//         if value > largest {
//             largest = value.clone();
//         }
//     }
//     largest
// }
//
// fn main() {
//     let integer = Point { x: 5, y: 6 };
//     let float = Point { x: 1.0, y: 4.0 };
//     let com = Point { x: 1, y: 2.1 };
//     let p1 = Point::new(1, 2.0);
//     println!("{}", p1.x);
//     let p2 = Point::new(3.0f32, 4.0f32);
//     let distance = p2.distance_from_origin();
//     println!("3 4 distance= {}", distance);
//
//     let p3 = integer.mixup(float);
//     println!("x:{}, y:{}", p3.x, p3.y);
//
//
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }


// use std::fmt::{Debug, Display, format};
//
// struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
//
// }
//
// pub trait Summary {
//     fn summarize_auth(&self) -> String;
//
//     fn summarize(&self) -> String {
//         format!("Read more from {} ...", self.summarize_auth())
//     }
// }
//
// impl Summary for Tweet {
//     fn summarize_auth(&self) -> String {
//         format!("@{}", self.username)
//     }
// }
//
//
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
//
// // item1 item2 可以使用不同的类型
// pub fn notify1(item1: impl Summary, item2: impl Summary) {}
//
// // 强调类型必须一致（类型必须相同）
// pub fn notify2<T: Summary>(item1: T, item2: T) {}
//
// // 多个约束，使用+
// pub fn notify3(item: impl Summary + Display) {}
//
// pub fn notify4<T: Summary + Display>(item: T) {}
//
// fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
//
// fn some_function_2<T, U>(t: T, u: U) where T: Display + Clone, U: Clone + Debug {}
//
//
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("ebooks"),
//         content: String::from("of course"),
//         reply: false,
//         retweet: false,
//     }
// }
//
//
// fn main() {
//     let tweet = Tweet {
//         username: String::from("ebooks"),
//         content: String::from("of course"),
//         reply: false,
//         retweet: false,
//     };
//
//     println!("{}", tweet.summarize())
// }


// use std::fmt::Display;
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     pub fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
//
//
//     // fn cmp_display(&self)
//     //     where T: PartialOrd + Display {
//     //     if self.x >= self.y {
//     //         println!("The largest member is x = {}", self.x);
//     //     } else {
//     //         println!("The largest member is y= {}", self.y);
//     //     }
//     // }
// }
//
// // 使用trait约束有条件的实现方式
// impl<T: PartialOrd + Display> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y= {}", self.y);
//         }
//     }
// }
//
//
// fn main() {
//     let p = Pair::new(10, 100);
//     p.cmp_display();
//
// }


// fn main() {
//     let r;
//     {
//         let x = 5;
//         // 引用，不转移所有权
//         //r = &x;
//
//         // 转移了所有权，没有问题
//         //r = x;
//     }
//     println!("{}", r);
// }


// // s1,s2,返回值指定一样的生命周期
// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1.len() > s2.len() {
//         return s1;
//     } else { return s2; }
// }
//
// // s1, 返回值 指定同一个生命周期,
// fn longest_1<'a>(s1: &'a str, s2: &str) -> &'a str {
//     s1
//     // if s1.len() > s2.len() {
//     //     return s1;
//     // } else { return s2; }
// }
//
// fn longest_2<'a>(s1: &str, s2: &str) -> &'a str {
//     let result = String::from("really");
//     result.as_str()
// }
//
//
// fn main() {
//     // let s1 = String::from("abcd");
//     // let s2 = "xyz";
//     // let s3 = longest(&s1, s2);
//     // println!("{:?}", s3);
//
//
//     // let s1 = String::from("long string");
//     // {
//     //     let s2 = String::from("xyz");
//     //     println!("{}", longest(&s1, &s2));
//     // }
//
//
//     // 借用检查不通过
//     let s1 = String::from("long string");
//     let result;
//     {
//         let s2 = String::from("xyz");
//         //result = longest(s1.as_str(), s2.as_str());
//         //result = longest_1(s1.as_str(), s2.as_str());
//         result = longest_2(s1.as_str(), s2.as_str());
//
//
//         println!("{}", result);
//     }
//     //println!("{}", result)
// }


// // 结构体持有引用，需要添加 生命周期标注
// struct ImportantExcept<'a> {
//     part: &'a str,
// }
//
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago ...");
//     let first_sentens = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcept { part: first_sentens };
// }


// // 同时使用泛型参数，trate约束，生命周期
// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, non: T) -> &'a str
//     where T: std::fmt::Display {
//     println!("longest_with_an_announcement: {}", non);
//     if x > y { x } else { y }
// }
//
// fn main() {
//     let s: &'static str = "I have a static lifetime";
//     println!("{}", s);
//
//
//     let s1 = String::from("aaaaaaaa");
//     let s2 = String::from("bbb");
//     let result = longest_with_an_announcement(s1.as_str(), s2.as_str(), "hello");
//     println!("{}", result);
// }


/////// 迭代器与闭包

// fn main() {
//     // fn add_one_v1(x: u32) -> u32 { x + 1 }
//     // let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     // let add_one_v3 = |x| { x + 1 };
//     // let add_one_v4 = |x| x + 1;
//
//
//     // let example_closure = |x| x;
//     // let s = example_closure(String::from("hello"));
//     // let n = example_closure(5);
//
// }


// use std::collections::HashMap;
// use std::thread;
// use std::time::Duration;
//
// fn main() {
//     // 闭包，真正需要用的时候，执行代码
//     // 使用闭包存储代码，重构
//
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;
//
//     generate_workout(
//         simulated_user_specified_value,
//         simulated_random_number,
//     )
// }
//
//
// fn generate_workout(intensity: u32, random_number: u32) {
//     // // 定义闭包
//     // let expensive_closure = |num: u32| -> u32{
//     //     println!("calculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     intensity
//     // };
//
//     // 使用泛型参数 & Fn trait 来存储闭包
//     let mut expensive_result = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });
//
//     if intensity < 25 {
//         println!("Today, {}", expensive_result.value(intensity));
//         println!("Next, {}", expensive_result.value(intensity))
//     } else {
//         if random_number == 3 {
//             println!("Take a break");
//         } else {
//             println!("Today, run fo {} minutes", expensive_result.value(intensity));
//         }
//     }
// }
//
// struct Cacher<T>
//     where T: Fn(u32) -> u32 {
//     calculation: T,
//     value: Option<u32>,
// }
//
// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32 {
//     pub fn new(calculation: T) -> Self {
//         Self {
//             calculation,
//             value: None,
//         }
//     }
//
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }
//
// #[test]
// fn call_with_differnet_values() {
//     let mut c = Cacher::new(|a| a);
//     let v1 = c.value(1);
//     let v2 = c.value(2);
//     // 参数不一样，返回值依然是原来的
//     assert_eq!(v2, 2);
// }


//
// fn main() {
//     // let x = 4;
//     // //fn equal_to_x(z: i32) -> bool { z + x }
//     // let equal_to_x = move |z| -> i32 { z + x };
//     // let y = 4;
//     // println!("{}", x);
//     // println!("{}", equal_to_x(y));
//     // println!("{}", x);
//
//
//     let x = vec![1, 2, 3];
//     let equal_to_x = move |z| z == x;
//     println!("can't use x here: {:?}", x);
// }

// enum List {
//     Cons(i32, List),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// fn main() {
//     let list = Cons(1,
//                     Box::new(Cons(2,
//                                   Box::new(Cons(3,
//                                                 Box::new(Nil))))));
// }

// fn main() {
//     // let x = 5;
//     // let y = &x;
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);
//
//     let x = 5;
//     let y = Box::new(x);
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// use std::ops::Deref;
//
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     pub fn new(field0: T) -> Self {
//         Self(field0)
//     }
// }
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// fn hello(name: &str) {
//     println!("Hello, {}", name);
// }
//
// fn main() {
//     // let x = 5;
//     // let y = MyBox(5);
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);
//
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
//     hello(&(*m)[..]);
//
//
//     let m2 = Box::new("Rust");
//     hello(&m2);
// }


// struct CustomSmartPointer {
//     data: String,
// }
//
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping {}", self.data);
//     }
// }
//
// fn main() {
//     let c = CustomSmartPointer { data: String::from("my stuff") };
//     let d = CustomSmartPointer { data: String::from("other stuff") };
//     println!("created");
//     //c.drop();
//     std::mem::drop(c);
//     println!("drop");
// }


// use std::rc::Rc;
// use crate::List::{Cons, Nil};
//
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
//
// fn main() {
//     //let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("a={}", Rc::strong_count(&a));
//     // clone
//     let b = Cons(3, Rc::clone(&a));
//     println!("a={}", Rc::strong_count(&a));
//
//     {
//         // clone
//         let c = Cons(4, Rc::clone(&a));
//         println!("a={}", Rc::strong_count(&a));
//
//     }
//     println!("a={}", Rc::strong_count(&a));
//
//     // Rc::clone(&a);
//     // Rc::clone(&a);
//     // Rc::clone(&a);
//     // println!("111 a={}", Rc::strong_count(&a));
// }



use std::cell::RefCell;

/// 持有数量：Rc<T>允许一份数据有多个所有者，而Box<T>和RefCell<T>都只 有一个所有者。
/// 编译检查：Box<T>允许在编译时检查的可变或不可变借用，Rc<T>仅允许编译时检查的不可变借用，RefCell<T>允许运行时检查的可变或不可变 借用。
/// 由于RefCell<T>允许我们在运行时检查可变借用，所以即便 RefCell<T>本身是不可变的，我们仍然能够更改其中存储的值

// use std::cell::RefCell;
//
// fn main() {
//     let a = 5;
//     // let x = &mut x;
//     let y = RefCell::new(a);
// }


// pub trait Messenger {
//     fn send(&self, msg: &str);
// }
//
// pub struct LimitTracker<'a, T: 'a + Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }
//
// impl<'a, T> LimitTracker<'a, T>
//     where T: Messenger {
//     pub fn new(messenger: &'a T, max: usize) -> Self {
//         Self { messenger, value: 0, max }
//     }
//
//     pub fn set_value(&self, value: usize) {
//         self.value = value;
//         let percentage_of_max = self.value as f64 / self.max as f64;
//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: you are over your qutal");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
//         } else {
//             self.messenger.send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use std::cell::RefCell;
//     use super::*;
//
//     use crate::{LimitTracker, Messenger};
//
//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }
//
//     impl MockMessenger {
//         pub fn new() -> MockMessenger {
//             MockMessenger { sent_messages: RefCell::new(vec![]) }
//         }
//     }
//
//     impl Messenger for MockMessenger {
//         fn send(&self, msg: &str) {
//             //self.sent_messages.push(String::from(msg));
//             self.sent_messages.borrow_mut().push(String::from(msg))
//         }
//     }
//
//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messager = MockMessenger::new();
//         let limit_tracker = LimitTracker::new(&mut mock_messager, 100);
//
//         limit_tracker.set_value(80);
//         assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
//     }
// }


// pub trait Draw {
//     fn draw(&self);
// }
//
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }
//
// impl Screen {
//     pub fn run(&self) {
//         for conponent in self.components {
//             conponent.draw();
//         }
//     }
// }

fn main() {
    let a = Box::new(5);
    println!("{}", a);

    let b = RefCell::new(5);
    println!("b = {:?}", b.borrow());
    {
        *b.borrow_mut() = 100;
        println!("b = {:?}", b.borrow());
    }
    println!("b = {:?}", b.borrow());

    let d = RefCell::new(vec![1, 2, 3]);
    {
        d.borrow_mut().push(10);
        println!("d = {:?}", d.borrow());
    }
    println!("d = {:?}", d.borrow());
}
