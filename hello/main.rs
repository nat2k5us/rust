// Rust is a new programming language
// Rust is in the TOP 20 Most Popular Programming languages 
// and competing with c and C++ in terms of speed 
// which accounts for its early adoption and acceptance

// Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
// `rustc` is the binary of rust 
// `cargo` is the package manager and build tool for rust

// A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.
mod datatypes;
fn main() {
    // Print text to the console
    println!("Hello World!");
    datatypes::execute();
    // datatypes::private_fn();
}
