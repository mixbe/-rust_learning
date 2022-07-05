extern crate crossbeam;
extern crate crossbeam_channel;

use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;

use lazy_static::lazy_static;
use std::sync::Mutex;
use anyhow::Result;
use rayon::prelude::*;


/// spwn a short-lived thread
fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        Some(max_l.max(max_r))
    }).unwrap()
}

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().unwrap();
    db.push(fruit.to_string());
    Ok(())
}


struct Person {
    age: u32,
}


#[cfg(test)]
pub mod tests {
    use crossbeam::channel::bounded;
    use crossbeam_channel::unbounded;
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;
    use super::*;

    #[test]
    pub fn test_01() {
        let arr = &[1, 25, -4, 10];
        let max = find_max(arr);
        assert_eq!(max, Some(25));
    }

    /// create a parallel pipeline
    ///
    // Source sent 0
    // Worker ThreadId(5) received 0.
    // Source sent 1
    // Source sent 2
    // Source sent 3
    // Worker ThreadId(4) received 1.
    // Worker ThreadId(5) received 2.
    // Sink received 0
    // Sink received 2
    // Sink received 4
    // Worker ThreadId(4) received 3.
    // Sink received 6
    #[test]
    pub fn test_02() {
        let (snd1, rcv1) = bounded(1);
        let (snd2, rcv2) = bounded(1);

        let n_msgs = 4;
        let n_workers = 2;

        crossbeam::scope(|s| {
            // producer thread
            s.spawn(|_| {
                for i in 0..n_msgs {
                    snd1.send(i).unwrap();
                    println!("Source sent {}", i);
                }
                drop(snd1);
            });
            // parallel processing by 2 threads
            for _ in 0..n_workers {
                let (sendr, recvr) = (snd2.clone(), rcv1.clone());
                s.spawn(move |_| {
                    thread::sleep(Duration::from_micros(500));
                    for msg in recvr.iter() {
                        println!("Worker {:?} received {}.", thread::current().id(), msg);
                        sendr.send(msg * 2).unwrap();
                    }
                });
            }

            // close the channel, otherwise sink will never
            // exit eth for-loop
            drop(snd2);

            for msg in rcv2.iter() {
                println!("Sink received {}", msg);
            }
        }).unwrap();
    }


    /// pass data between two threads
    #[test]
    pub fn test_03() {
        let (snd, rcv) = unbounded();
        let n_msgs = 5;
        crossbeam::scope(|s| {
            s.spawn(|_| {
                for i in 0..n_msgs {
                    snd.send(i).unwrap();
                    println!("snd: {}", i);
                    thread::sleep(Duration::from_micros(100));
                }
            });
        }).unwrap();

        for _ in 0..n_msgs {
            let msg = rcv.recv().unwrap();
            println!("Received {}", msg);
        }
    }

    /// maintain global mutable state
    #[test]
    pub fn test_04() -> Result<()> {
        insert("apple")?;
        insert("orange")?;
        insert("peach")?;
        {
            let db = FRUIT.lock().unwrap();

            db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
        }
        insert("grape")?;
        Ok(())
    }


    /// mutate the elements of an array in parallel
    #[test]
    pub fn test_05() {
        let mut arr = [0, 7, 9, 11];
        arr.par_iter_mut().for_each(|p| *p -= 1);
        println!("{:?}", arr);
    }

    #[test]
    pub fn test_06() {
        let mut vec = vec![2, 4, 6, 8];

        assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
        assert!(vec.par_iter().all(|n| (*n % 2) == 0));
        assert!(!vec.par_iter().any(|n| *n > 8));
        assert!(vec.par_iter().all(|n| *n <= 8));

        vec.push(9);


        assert!(vec.par_iter().any(|n| (*n % 2) != 0));
        assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
        assert!(vec.par_iter().any(|n| *n > 8));
        assert!(!vec.par_iter().all(|n| *n <= 8));
    }


    /// Search items using given predicate in parallel
    #[test]
    pub fn test_07() {
        let v = vec![6, 2, 1, 9, 3, 8, 11];
        let f1 = v.par_iter().find_any(|&&x| x == 9);
        let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
        let f3 = v.par_iter().find_any(|&&x| x > 8);

        assert_eq!(f1, Some(&9));
        assert_eq!(f2, Some(&8));
        assert!(f3 > Some(&8));
    }

    #[test]
    pub fn test_08() {
        // let mut vec = vec![String::new(); 100_000];
        // vec.par_iter_mut().for_each(|p| {
        //     let mut rng = thread_rng();
        //     *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
        // });

        let mut vec = vec!["456", "789", "123"];
        println!("{:?}", vec);
        vec.par_sort_unstable();
        println!("{:?}", vec);
    }

    /// map-reduce in parallel
    #[test]
    pub fn test_09() {
        let v: Vec<Person> = vec![
            Person { age: 23 },
            Person { age: 19 },
            Person { age: 42 },
            Person { age: 17 },
            Person { age: 17 },
            Person { age: 31 },
            Person { age: 30 },
        ];

        let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
        let sum_over_30 = v.par_iter()
            .map(|x| x.age)
            .filter(|&x| x > 30)
            .reduce(|| 0, |x, y| x + y);

        let alt_sum_30: u32 = v.par_iter()
            .map(|x| x.age)
            .filter(|&x| x > 30)
            .sum();

        let avg_over_30 = sum_over_30 as f32 / num_over_30;
        let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

        assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
        println!("The average age of people older than 30 is {}", avg_over_30);
    }
}
