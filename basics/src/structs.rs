struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct BasicColor(u8, u8, u8);

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age: age
        }
    }

    fn born(&self) -> u32 {
        2021 - self.age
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    fn to_tuple(&self) -> (String, u32, u32) {
        (self.name.to_string(), self.age, self.born())
    }
}

pub fn run() {
    let mut c = Color{red: 255, green: 0, blue: 0};
    c.green = 255;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut d = BasicColor(255, 0, 0);
    d.2 = 155;
    println!("BasicColor: {} {} {}", d.0, d.1, d.2);

    let mut p = Person::new("Juan", 38);
    p.set_name("Juan Goyes");
    println!("Person: {} - {} - {}. Tuple: {:?}", p.name, p.age, p.born(), p.to_tuple())
}