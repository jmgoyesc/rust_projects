pub fn run() {
    //vectors
    let mut numbers: Vec<i32> = vec![2,4,6,8,10];
    numbers.push(12);
    numbers.push(14);
    numbers.push(16);

    for x in numbers.iter_mut() {
        *x = *x * *x
    }
    println!("{:?}", numbers);
}