pub fn run() {
    for number in 1..100 {
        if number % 3 == 0 {
            print!("Fizz");
        }
        if number % 5 == 0 {
            print!("Buzz");
        }
        println!(". {}", number)
    }
}