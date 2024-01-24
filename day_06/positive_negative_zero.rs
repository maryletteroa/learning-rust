// If- Else - If Else demo -- Positive Negative Zero
fn main() {
    //Declare mutable string to stroe the input string form
    let mut num_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number: ");
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();
    //Parse the input and try to convert string  equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    // If the number is greater than 0, means positive
    if num > 0 {
        println!("\n{} is positive.\n", num);
    } 
    //If the number is less than 0, means negative
    else if num < 0 {
        println!("\n{} is negative.\n", num);
    }
    // If the number is neither positive nor negative, means it is 0
    else {
        println!("\n{} is zero.\n", num);
    }
}