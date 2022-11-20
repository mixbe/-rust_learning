use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    Unspecified = 0,
    Mail = 1,
    Female = 2,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    age: u8,
    gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    // 从文件中加载对象
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user: User = serde_json::from_str(&data)?;
        Ok(user)
    }

    // 保存对象到文件
    pub fn persist(&self, file_path: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(file_path)?;

        let data = serde_json::to_string(self)?;
        // write an entire buffer to this writer
        file.write_all(data.as_bytes())?;
        Ok(data.len())
    }
}

impl User {
    fn default() -> Self {
        User::new("Unknow user".into(), 0, Gender::Unspecified)
    }
}


fn main() {
    let user = User::default();
    println!("user: {:?}", user);

    let path = "/tmp/user1";
    user.persist(path).unwrap();

    let user1 = User::load(path).unwrap();
    println!("user1: {:?}", user1);
}