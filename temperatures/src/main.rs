use std::error::Error;
use std::fmt;

fn main() {
    let input = read_input();
    let output = input.transform();
    println!("Temprature. Input: {}{} - Output: {}{}", input.value, input.scale, output.value, output.scale);
}

fn read_input() -> Temperature {
    loop {
        println!("Write temperature in Fahrenheit or Celsius: (###F or ###C)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input");

        match Temperature::new(input) {
            Ok(v) => {
                return v
            },
            Err(_) => {
                continue;
            }
        }
    }
}

struct Temperature {
    scale: char,
    value: f32,
}

impl Temperature {
    fn new(txt: String) -> std::result::Result<Temperature, MyError> {
        // read input until parse is valid
        if txt.len() < 2 {
            return Err(MyError::new("Input too short"));
        }
        let (value, scale) = txt.split_at(txt.len() - 2);
        let scale = match scale.chars().next() {
            None => {
                return Err(MyError::new("Unable to read"))
            }
            Some(v) => v
        };
        if scale != 'C' && scale != 'F' {
            return Err(MyError::new("Value does not contain 'F' nor 'C'"));
        }
        let value:f32 = match value.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                return Err(MyError::new("Invalid input"))
            }
        };
        Ok(Temperature{scale, value})
    }

    fn transform(&self) -> Temperature {
        if self.scale == 'F' {
            Temperature{scale: 'C', value: (self.value - 32.0) * 5.0/9.0}
        } else {
            Temperature{scale: 'F', value: (self.value * 9.0/5.0) + 32.0}
        }
    }
}

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(details: &str) -> MyError {
        MyError{details: details.to_string()}
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}