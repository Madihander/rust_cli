use std::io;
use std::env;
fn main() {
    println!("Hello user!");
    println!("It is rust cli \n");

    let pattern = env::args().nth(1).expect("no pattern given");
    let path = env::args().nth(2).expect("no path given");

    //let mut cmd = String::new();
    //println!("Enter command: ");
    
    //let _ = io::stdin().read_line(&mut cmd); 
    //println!("Command: {}",cmd);

}
