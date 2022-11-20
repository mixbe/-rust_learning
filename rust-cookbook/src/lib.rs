pub mod compression;
pub mod algorithms;
pub mod command_line;
pub mod concurrency;
pub mod cryptography;
pub mod date_and_time;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_main(){
        let mut s = String::from("foo");
        println!("{}", s)
    }
}
