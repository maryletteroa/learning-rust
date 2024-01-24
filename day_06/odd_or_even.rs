//If demo -- odd or even
fn main() {
    //Declare mutable string to stroe the input string form
    let mut num_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number: ");
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();
    //Parse the input and try to convert string  equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    //Divide num by two, check if the remainder is 1, which means the number is odd
    if num % 2 == 1 {
        println!("{} is odd.\n", num);
    }
    //If the remainder is zero, means the number is even
    else {
        println!("{} is even.\n", num);
    }
}