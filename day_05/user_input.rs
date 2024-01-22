//User input demo
fn main() {
    // Declare a mutable string to store the input
    let mut text = String::new();
    // Prompt the user to enter something
    println!("\nEnter some text:");
    //Read the input using stdin
    std::io::stdin().read_line(&mut text).unwrap();
    //Print whatever was read
    println!("\nYou have entered: {}", text);
}