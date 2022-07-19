fn main() {
    let a = 42u8;
    let b = 42.1f64;

    // 不会做隐式转换
    println!("a+b = {}", a + b);
}
