pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn average(&self) -> f64 { self.average }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        }
    }


    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut col = AverageCollection {
        list: vec![],
        average: 0.0,
    };
    col.add(1);
    println!("{}", col.average);

    col.add(2);
    println!("{}", col.average);

    col.add(3);
    println!("{}", col.average);

    let pop = col.remove().unwrap();
    println!("{}", pop);
    println!("{}", col.average);
}

////////////////////////////////


