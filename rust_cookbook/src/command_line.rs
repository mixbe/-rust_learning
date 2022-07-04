#[cfg(test)]
pub mod tests {
    use clap::{App, Arg};
    use super::*;

    #[test]
    pub fn test_01() {
        let matches = App::new("My test Program")
            .version("1.0.0")
            .author("mixbee@hack.gov")
            .about("Teches argument parsing")
            .arg(
                Arg::with_name("file")
                    .short('f')
                    .long("file")
                    .takes_value(true)
                    .help("A cool file"))
            .arg(
                Arg::with_name("num")
                    .short('n')
                    .long("number")
                    .takes_value(true)
                    .help("Five less than your favorite number"))
            .get_matches();

        let myfile = matches.value_of("file").unwrap_or("input.txt");
        println!("The file passed is: {}", myfile);
        let num_str = matches.value_of("num");
        match num_str {
            None => println!("No idea what your favorite numer is."),
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) => println!("Your faorite number must be {}.", n + 5),
                    Err(_) => println!("That's not a number {}", s)
                }
            }
        }
    }
}