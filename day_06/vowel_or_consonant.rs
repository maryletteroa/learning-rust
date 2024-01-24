//Match Demo - Multiple Constant Expressions
fn main() {
    //Declare mutable string to store the input in string form
    let mut char_str = String::new();
    //Prompt the user to enter an alphabet
    println!("Enter an alphabet:");
    //Read the input using stdin, store the string input to char_str
    std::io::stdin().read_line(&mut char_str).unwrap();
    //Parse the input
    let c:char = char_str.trim().parse().unwrap();
    //Match for c
    match c {
        'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => {println!("\n{} is a vowel.\n", c)},
        _ => {println!("\n{} is a consonant",c)}
    }
}