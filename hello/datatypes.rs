  #[derive(Debug)]

// mod datatypes {
//   pub fn public_fn(){}
//   fn private_fn(){}
// }
// }
  enum Status {
    ACTIVE,
    INACTIVE,
    }


fn private_fn(){
  println!("Cannot be accessed outside the module")
}
pub fn execute() {
    // data type char
    let x = 'a';
    let b = 'y';
    println!();
    print!("x = {},\nb = {}", x, b);
    println!();
    // int
    let i = 7;
    println!("integer i = {}", i);
    // float
    let f = 1.34;
    println!("float f = {}", f);
    // bool
    let yes = true;
    let no = false;
    println!("bool yes = {}, no = {}", yes, no);

    //mutable - by default all var are immutable
    // to make a var mutable add mut after let for ex. "let mut me = nat"
    let mut me = "joe ".to_string();
    let you = "doe".to_string();
    me.push_str(&you);
    println!("me = {}", me);
    //const
    const RUST_IS: &str = "Rust is speed";
    println!("RUST_IS = {}", RUST_IS);
    // specify data type explicitly
    let score: i32 = 100;
    let x_value: f32 = 32.1;
    // enum
    let active = Status::ACTIVE;
    let inactive = Status::INACTIVE;
    println!("{:?}", active);
    println!("{:?}", inactive);
    // named args
    println!(" {name} likes to play {game}", name="Nat", game="Tennis");

    // hex dec oct bin traits
    println!("{:?} {:b} {:e} {:o} {:x}",110, 110,110,110,110);
}
