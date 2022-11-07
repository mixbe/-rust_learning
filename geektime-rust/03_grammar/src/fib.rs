fn fib_loop(n: u8) {
    let mut a = 1u8;
    let mut b = 1u8;
    let mut i = 2u8;
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("{:?}", b);
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1u8, 1u8, 2u8);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("{:?}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1u8, 1u8);
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("{}", c);
    }
}


fn main() {
    let n = 10;
    fib_loop(n);
    fib_for(n);
    fib_while(n);
}