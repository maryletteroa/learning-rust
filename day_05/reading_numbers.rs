//User input Demo - numbers
fn main(){
    //Declare mutable strings to store the input in string form
    let mut num1_str = String::new();
    let mut num2_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number:");
    //Read the input using stdin, store the string input in num1_str
    std::io::stdin().read_line(&mut num1_str).unwrap();
    //Prompt the user to enter a number
    println!("Enter another number:");
    //Read the input string stdin, store the string input in num2_str
    std::io::stdin().read_line(&mut num2_str).unwrap();
    //Parse the input and try to convert string equivalent numbers to int
    let num1:isize = num1_str.trim().parse().unwrap();
    let num2:isize = num2_str.trim().parse().unwrap();
    //Add both numbers
    let sum:isize = num1 + num2;
    //Print num1 num2 and sum
    println!("\nnum1 = {}, num2 = {}, \nsum = {}\n", num1, num2, sum);
}