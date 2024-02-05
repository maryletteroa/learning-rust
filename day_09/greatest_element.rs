//Greatest Elemetn of an Array
fn main() {
    //Declare mutable string to store the input in string form
    let mut num_str = String::new();
    //Declare a loop counter
    let mut i = 0;
    //Declare greatest
    let mut greatest:isize = -999;
    //Declare a mutable array of 5 integers, fill with zeros
    let mut num_array:[isize;5] = [0; 5];
    //Loop from 0 to 4, five times
    while i < 5 {
        //Prompt the user to enter a number
        println!("\nEnter a number at index {}: ", i);
        //Clear num_str so that previous input is flused
        num_str.clear();
        //Read the input using stdin ,store the string input in num_str
        std::io::stdin().read_line(&mut num_str).unwrap();
        //Parse the input and try to convert string equivalent numbers to int
        //Place it into an array
        num_array[i] = num_str.trim().parse().unwrap();
        //Check if the current number is greater than greatest
        if num_array[i] > greatest {
            //Assign greatest to current element
            greatest = num_array[i];
        }
        //Incrementi
        i += 1;
    }
    //Print array and greatest
    println!("\nnum_array: {:?} \nGreatest: {}", num_array, greatest);
}