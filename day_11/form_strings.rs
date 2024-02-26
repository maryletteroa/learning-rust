//String demo - Forming Strings
fn main() {
    //Declare a string of literal type
    let name1:&str = "Orin";
    //Declare an empty string of object ttype
    let mut name2 = String::new();
    //Assign some value using a push_str function
    name2.push_str("Deb");
    //Create a string using from method
    let name3 = String::from("Lola");
    //Create a variable with &str type converted to object type
    let name4 = "Jade".to_string();
    //Create a float variable
    let num:f64 = 54.75;
    //Convert the float variable to a stringu using to_string function
    let num_str = num.to_string();
    //Print all strings and their sizes
    println!("\nname1:{}\tLength: {}", name1, name1.len());
    println!("\nname2:{}\tLength: {}", name2, name2.len());
    println!("\nname3:{}\tLength: {}", name3, name3.len());
    println!("\nname4:{}\tLength: {}", name4, name4.len());
    println!("\nnum_str: {} \tLength: {}\n", num_str, num_str.len());
}