use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("starting");
}

async fn read_from_file1() -> String {
    println!("read_from_file1 .......");
    sleep(Duration::new(4, 0));
    println!("{:?}", "Processing file 1");
    String::from("Hello, there from file 1")
}

async fn read_from_file2() -> String {
    println!("read_from_file2 .......");
    sleep(Duration::new(2, 0));
    println!("{:?}", "Processing file 2");
    String::from("Hello, there from file 2")
}


use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Instant;

fn read_from_file_v2_1() -> impl Future<Output=String> {
    async {
        println!("read_from_file1 .......");
        sleep(Duration::new(4, 0));
        println!("{:?}", "Processing file 1");
        String::from("Hello, there from file 1")
    }
}

fn read_from_file_v2_2() -> impl Future<Output=String> {
    async {
        println!("read_from_file2 .......");
        sleep(Duration::new(2, 0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}


struct ReadFileFuture {}

/*
自定义Future
 */
impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Tokio! Stop polling me");
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

struct AsyncTime {
    expiration_time: Instant,
}

impl Future for AsyncTime {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() > self.expiration_time {
            println!("Hello: it's time for Future 1");
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            println!("Hello, it's not yet time for Future 1");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            std::thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    std::thread::sleep(expiration_time - current_time);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_async_v1() {
        let h1 = tokio::spawn(async {
            let _fil1_connects = read_from_file1().await;
        });

        let h2 = tokio::spawn(async {
            let _fil2_connects = read_from_file2().await;
        });

        let _ = tokio::join!(h1, h2);
    }

    /*
    功能上类似v1
     */
    #[tokio::test]
    async fn test_async_v2() {
        let h1 = tokio::spawn(async {
            let _fil1_connects = read_from_file_v2_1().await;
        });

        let h2 = tokio::spawn(async {
            let _fil2_connects = read_from_file_v2_2().await;
        });

        let _ = tokio::join!(h1, h2);
    }


    /*
    利用Tokio库尝试理解Future(得main执行)
     */
    #[tokio::test]
    async fn test_async_v3() {
        println!("Hello before reading file!");
        let h1 = tokio::spawn(async {
            let future1 = ReadFileFuture {};
            future1.await
        });

        let h2 = tokio::spawn(async {
            let fil2_connects = read_from_file_v2_2().await;
            println!("{:?}", fil2_connects);
        });

        let _ = tokio::join!(h1, h2);
    }

    #[tokio::test]
    async fn test_async_v4() {
        println!("------");
        let h1 = tokio::spawn(async {
            let future1 = AsyncTime {
                expiration_time: Instant::now() + Duration::from_micros(4000)
            };
            println!("{:?}", future1.await)
        });

        let h2 = tokio::spawn(async {
            let file2_contents = read_from_file_v2_2().await;
            println!("{:?}", file2_contents);
        });
        let _ = tokio::join!(h1, h2);
    }
}


