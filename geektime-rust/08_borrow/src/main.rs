fn main() {
    // let data = vec![1, 2, 3, 4];
    // let data1 = &data;
    // // 值的地址是什么？引用的地址又是什么？
    // // addr of value: 0x7ff7b9bbed88(0x7ff7b9bbed88), addr of data 0x7ff7b9bbee28, data1: 0x7ff7b9bbeda0
    // println!("addr of value:  {:p}({:p}), addr of data {:p}, data1: {:p}", &data, data1, &&data, &data1);
    // println!("sum of data1: {}", sum(data1));
    //
    // // 堆上数据的地址是什么？
    // // addr of items: [0x600002a50040, 0x600002a50044, 0x600002a50048, 0x600002a5004c]
    // println!("addr of items: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3]);


    // let mut data = vec![1,2,3];
    // for item in data.iter_mut(){
    //     // 多个引用共存
    //     data.push(*item +1);
    // }


    // // 同时有一个可变引用，若干个只读引用
    // let mut data = vec![1,2,3];
    // let data1 = vec![&data[0]];
    //
    // println!("data[0]: {:p}", &data[0]);
    //
    // // 有可能堆上的空间不够，会重新分配
    // // for i in 0..10 {
    // //     data.push(i);
    // // }
    //
    // println!("data[0]: {:p}", &data[0]);
    // println!("boxed: {:p}", &data1);


    let mut arr = vec![1,2,3];
    //let last = arr.last();
    let last = *arr.last().unwrap();
    arr.push(4);

    println!("last: {:?}", last);



}

fn sum(data: &[u32]) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().sum()
}