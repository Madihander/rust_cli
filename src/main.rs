use std::io;
fn main() {
    println!("Hello user!");
    println!("It is rust cli \n");
    let mut cmd = String::new();
    println!("Enter command: ");
    let _ = io::stdin().read_line(&mut cmd); 
    println!("Command: {}",cmd)
}
