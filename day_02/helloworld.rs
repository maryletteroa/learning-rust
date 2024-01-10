// Hello World! Program

// Usage of print and println
// Main function
fn main() {
    // println! macro to print a message on a new line
    println!("Hello");
    println!("World!");
    // print! macro to print a message on the same line
    print!("Hello");
    print!("World");

    // escape sequuence
    println!("\nThis should be printed on the first line.\nAnd this on the next one!");
    print!("\nLeaving another line.\tThis is what a tab-space indent looks like.\nHappy Programming!\n");
}