//Match range demo
fn main() {
    //Declare mutable string to store the input in string form
    let mut num_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number between 1 and 1000: ");
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();
    //Parse the input and try to convert string equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    //Check which range does num belong to using match
    match num {
        1..=99 => {println!("\n{} is between 1 and 99.\n", num)},
        100..=199 => {println!("\n{} is between 100 and 199.\n", num)},
        200..=299 => {println!("\n{} is between 200 and 299.\n", num)},
        300..=399 => {println!("\n{} is between 300 and 399.\n", num)},
        400..=499 => {println!("\n{} is between 400 and 499.\n", num)},
        500..=599 => {println!("\n{} is between 500 and 599.\n", num)},
        600..=699 => {println!("\n{} is between 600 and 699.\n", num)},
        700..=799 => {println!("\n{} is between 700 and 799.\n", num)},
        800..=899 => {println!("\n{} is between 800 and 899.\n", num)},
        900..=1000 => {println!("\n{} is between 900 and 1000.\n", num)}
        _ => println!("\n{} is either less than 1 or greather than 1000.\n", num)
    }
}
