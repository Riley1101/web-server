use std::vec;

#[derive(Debug)]
enum Shirts {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Store {
    shirts: Vec<Shirts>,
}

impl Store {
    fn get_all(&self) {
        for colors in &self.shirts {
            println!("{:?}", colors);
        }
    }

    fn get_count(&self) -> Vec<i32> {
        let mut blue_count = 0;
        let mut green_count = 0;
        let mut red_count = 0;
        for shirt in &self.shirts {
            match shirt {
                Shirts::Red => red_count += 1,
                Shirts::Green => green_count += 1,
                Shirts::Blue => blue_count += 1,
            }
        }
        return vec![red_count, green_count, blue_count];
    }
}

fn main() {
    let store = Store {
        shirts: vec![Shirts::Red, Shirts::Blue, Shirts::Red, Shirts::Green],
    };
    store.get_all();
}
