fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


pub trait Summary {
    fn summarize(&self) -> String;
}


fn main() {
    println!("xx");
}