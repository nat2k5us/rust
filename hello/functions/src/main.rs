fn main() {
    my_func();
    println!("{}",add_ints(12, 23));
    my_func();

    let x = {
        let a = 10;
        let b = 20;
        a + 5 + b*5
    };
    println!("{}",x);
}

fn add_ints(a: i32, b: i32) -> i32 {
    a + b
}

fn my_func(){
    println!("This is my test function")
}
#[allow(dead_code)]
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}