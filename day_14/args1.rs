//Command line arguments demo 1
//Use env module from the Rust standard library std
use std::env;
fn main() {
    //Call collect() function to fetch all the arguments
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments: \n{:?}\nTotal number of arguments: {}\nArguments passed: {}", args, args.len(), (args.len()-1));
} 