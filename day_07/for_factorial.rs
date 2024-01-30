//for demo -- factorial
fn main() {
    //Declare mutable string to store the input in string form
    let mut num_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number: ");
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();
    //Parse the input and try to convert string equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    let mut factorial: isize = 1;
    if num >= 0 {
        for i in 1..=num {
            factorial = factorial * i;
        }
        println!("\nFactorial of {} is {}", num, factorial);
    }
    else {
        println!("\nFactorial of a negative number cannot be calculated\n");
    }
}