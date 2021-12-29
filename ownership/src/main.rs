fn main() {
    let s = String::from("hello");
    takes_ownership(s); // after this line, s use is invalid because is move the ownership to the funciton

    // println!("Invalid use o s {}", s); //s value borrowed      

    let x = 5;
    makes_copy(x); // i32 implements the trait Copy, and it wil copy the memory from the stack (knwon size). No Drop implemented. Use x after this line is valid

    println!("Valid use of x because Copy: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}
