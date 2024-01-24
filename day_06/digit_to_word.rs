//Match demo
fn main() {
    //Declare mutable string to stroe the input string form
    let mut num_str = String::new();
    //Prompt the user to enter a number
    println!("Enter a number: ");
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();
    //Parse the input and try to conver string equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    let num_str = match num {
        0 => "ZERO",
        1 => "ONE",
        2 => "TWO",
        3 => "THREE",
        4 => "FOUR",
        5 => "FIVE",
        6 => "SIX",
        7 => "SEVEN",
        8 => "EIGHT",
        9 => "NINE",
        _ => "An Invalid Input!"
    };
    println!("\nYou have entered: {}\n", num_str);
}