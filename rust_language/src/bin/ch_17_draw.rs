pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub componnets: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for componnet in self.componnets.iter() {
            componnet.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button");
    }
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox");
    }
}


fn main() {
    let screen = Screen {
        componnets: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),

            }),
        ],
    };

    screen.run();
}