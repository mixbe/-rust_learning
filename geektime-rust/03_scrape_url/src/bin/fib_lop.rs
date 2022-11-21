fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {}

fn fib_for(n: u8) {}

fn main() {
    fib_loop(6);
}