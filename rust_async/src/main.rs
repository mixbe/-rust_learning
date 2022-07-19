
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: String,
}

impl User {
    pub fn new(name: String, age: u8, gender: String) -> Self { Self { name, age, gender } }
}





fn main() {
    println!("Hello, world!");
}
