// use crate::List::{Cons, Nil};

/// 内容
/// Box<T>  在heap内存上分配值
/// Rc<T>  启用多重所有权的引用计数类型
/// Ref<T> RefMut<T> 通过RefCell<T>


// enum List {
//     Cons(i32, List),
//     Nil,
// }

// fn main() {
//     // let b = Box::new(5);
//     // println!("{}", b);
//
//     //let list = Cons(1, Cons(2, Cons(3, Cons(4, Nil))));
// }

struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    let _ = s;
    print!("2");
}