use anyhow::Result;
use std::num::FpCategory::Normal;
use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand_distr::{Distribution as Distribut, Normal as Normal2, NormalError};


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate random numbers
    #[test]
    fn test_01() {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        println!("Rnadom u8: {}", n1);
        println!("Rnadom u16: {}", n2);

        println!("Rnadom u32: {}", rng.gen::<u32>());
        println!("Rnadom i32: {}", rng.gen::<i32>());
        println!("Rnadom float: {}", rng.gen::<f64>());
    }

    /// generate random numbers within a range
    #[test]
    pub fn test_02() {
        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(0..10));
        println!("Float: {}", rng.gen_range(0.0..10.0));

        // 分布区间
        let die = Uniform::from(1..7);
        loop {
            let throw = die.sample(&mut rng);
            println!("Roll the die: {}", throw);
            if throw == 6 {
                break;
            }
        }
    }

    /// generate random numbers with given distribution
    #[test]
    pub fn test_03() -> Result<(), NormalError> {
        let mut rng = thread_rng();
        let normal = Normal2::new(2.0, 3.0)?;
        let v = normal.sample(&mut rng);
        println!("{} is from a N(2, 9) distribution", v);
        Ok(())
    }

    /// Generate random values of a custom type
    #[test]
    pub fn test_04() {
        let mut rng = rand::thread_rng();
        let rand_tuple = rng.gen::<(i32, bool, f64)>();
        let rand_point: Point = rng.gen();
        println!("Random tuple: {:?}", rand_tuple);
        println!("Random Point: {:?}", rand_point);
    }

    /// Craete random passwords from a set of alphanumeric characters
    #[test]
    pub fn test_05() {
        // a-z, A-Z, 0-9
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        println!("{}", rand_string)
    }

    /// create random passwords from a set of user-defined characters
    #[test]
    pub fn test_06() {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 30;
        let mut rng = rand::thread_rng();
        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        println!("{:?}", password);
    }
}