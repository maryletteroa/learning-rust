//If demo
fn main() {
    //Declare mutable string to stroe the input string form
    let mut num_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number: ");
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();
    //Parse the input and try to convert string  equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    //Check if the number is greater than 10
    if num > 10 {
        println!("\n{} is greater than 10.\n", num);
    }
}