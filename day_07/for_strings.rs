//For demo
fn main() {
    //Declare mutable string to store the input in string form
    let mut str1 = String::new();
    //Prompt the user to enter a string
    println!("Enter a something:");
    //Read the input using stdin, store the input in str1
    std::io::stdin().read_line(&mut str1).unwrap();
    for alphabet in str1.chars() {
        print!("\n{}", alphabet);
    }
}