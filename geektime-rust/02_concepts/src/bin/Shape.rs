struct Rectangle {
    a: f64,
    b: f64,

}


struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}


trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        todo!()
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        todo!()
    }
}

enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}


impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => { r.area() }
            Shape::Cir(c) => { c.area() }
            Shape::Tri(t) => { t.area() }
        }
    }
}


fn main() {
    println!("")
}