fn main() {
    println!("Hello, world!");
}

// Functions that you wish to access from Javascript
// must be marked as no_mangle

#[no_mangle]
pub fn run_example() -> i32 {
    return 0;
}
