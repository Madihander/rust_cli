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
    // Get all content of file from the path
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");
    // Iterating by every line in file
    for line in content.lines() {
        // find line that contain pattern
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
    //println!("Pattern: {}",args.pattern);
    //println!("Pattern: {:?}",args.path);
}
