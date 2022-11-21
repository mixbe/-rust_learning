1. 有一个指向某个函数的指针，如果将其解引用成一个列表，然后往列表中插入一个元素，请问会发生什么？（对比不同语言，看看这种操作是否允许，如果允许会发生什么

2. 要构造一个数据结构 Shape，可以是 Rectangle、 Circle 或是 Triangle，这三种结构见如下代码。请问 Shape 类型该用什么数据结构实现？怎么实现？

```rust
struct Rectangle {
    a: f64,
    b: f64,

}


struct Circle {
    r: f64
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64
}


```

3. 对于上面的三种结构，如果我们要定义一个接口，可以计算周长和面积，怎么计算？