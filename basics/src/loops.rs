pub fn run() {
    let mut count = 1;
    println!("Loops: {0} - {0}", count);
    for x in 1..10 {
        println!("number: {}", x);
    }
    count += 1;
    println!("end loops: {count}", count = count);
}