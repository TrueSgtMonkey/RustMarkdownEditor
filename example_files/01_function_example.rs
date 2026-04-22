use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    // how to print to console
    println!("{}", test_function());
}

// can add functions and call them regardless of placement
fn test_function() -> i32 {
    let num = 32;
    num
}