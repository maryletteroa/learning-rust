//Arrays demo - while loop
fn main() {
    //Declare string array, specify type and size
    let name_array:[&str;6] = ["Theo", "Nina", "Eva", "Otis","Vin", "Sidi"];
    //Declar loop counter
    let mut count = 0;
    println!("\nname_array printed using while loop:\n");
    //Loop from 0 to name_array.len()
    while count < name_array.len() {
        //Print value at current index
        println!("\nElement at {}: {}", count, name_array[count]);
        //Increment count
        count = count + 1;
    }
    // count + 1 will be the size of the array
    println!("\n\nArray size: {}", count);
}