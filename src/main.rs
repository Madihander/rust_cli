use std::env;
use clap::Parser;

#[derive(Parser)]
struct CLI{
    // The pattern to loor for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}
fn main() {
    println!("Hello user!");
    println!("It is rust cli \n");

    let args = CLI::parse();
    
}
