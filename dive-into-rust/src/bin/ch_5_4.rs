trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash");
    }
}


// 完整函数调用
fn main() {
    let me = Chef;
    // multiple `start` found
    //me.start();

    <dyn Cook>::start(&me);
    <dyn Wash>::start(&me);
    <Chef as Wash>:: start(&me);
}