const MY_PI: f64 = std::f64::consts::PI;


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


enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.r * self.r * MY_PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        // 根据海伦公式求三角形面积
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c) / 2.0;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }
}


impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area()
        }
    }
}


fn main() {
    let rec = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    let shape = Shape::Tri(rec);
    println!("ares is:{}", shape.area())
}
