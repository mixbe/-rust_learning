use std::collections::HashMap;
use std::thread;

pub fn value_tour() {
    // modify value
    let mut total = 0usize;
    total += 1;

    // pass to function
    let name = "hello".to_string();
    print_my_name(name);


    // pass by ref
    let mut map: HashMap<String, String> = HashMap::new();
    let my_map = &mut map;


    // multithreaded
    let mut data = vec![1, 2, 3];
    thread::spawn(move || {
        data.push(5);
    });

}

fn print_my_name(name: String) {
    println!("{}", name);
}

fn print_map(map: &HashMap<String, String>) {
    todo!();
}

fn insert_map(map: &mut HashMap<String, String>) {
    todo!();
}

fn main() {
    println!("hello world");
}