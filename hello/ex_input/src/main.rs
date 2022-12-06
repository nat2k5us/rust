use std::io;
fn main() {
    let mut name = String::new();
    println!("Hello, What is you name?");
    io::stdin().read_line(&mut name).expect("throw an exception on fail");
    println!("Greetings {}", name);

    // Integer input
    let mut age = String::new();
    println!("Hello, What is you age?");
    io::stdin().read_line(&mut age).expect("age old exception thrown"); 
    let int_age = age.trim();
    match int_age.parse::<u32>() {
        Ok(i) => println!("your age is: {}", i),
        Err(..) => println!("your age should be integer: {}", int_age),
    };
}
