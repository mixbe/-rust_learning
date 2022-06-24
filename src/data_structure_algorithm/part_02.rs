// 递归(recursion)实战

// f(n) = f(n-1) + f(n-2)
// f(1) = 1
// f(0) = 0
pub fn fibonacci(num: usize) -> usize {
    if num == 0 { return 0; }
    if num == 1 { return 1; }
    return fibonacci(num - 1) + fibonacci(num - 2);
}


#[test]
fn test_fibonacci() {
    // 0 1 1 2 3 5 8
    let num = 5;
    print!("{} fibonacci is {} ", num, fibonacci(num));

    // 0 1 1 2 3 5 8
    let num2 = 6;
    print!("{} fibonacci is {} ", num2, fibonacci(num2));
}


// 阶乘 n! = n*(n-1)*(n-2).....
pub fn factorial(num: usize) -> usize {
    if num <= 1 {
        return 1;
    }
    return num * factorial(num - 1);
}

#[test]
pub fn test_factorial() {
    // 5 * 4 * 3 * 2 * 1 = 120
    let num = 5;
    print!("!{} = {}", num, factorial(num));
}