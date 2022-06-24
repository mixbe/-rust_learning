
/**
 *   二维数组打印
 */
pub fn print_array(vec: Vec<[usize; 2]>) {
    for items in vec.iter() {
        for item in items {
            println!("{}", item);
        }
    }
}

#[test]
fn test_print_array() {
    let v1 = vec![[1, 2], [3, 5], [8, 0]];
    print_array(v1);
}