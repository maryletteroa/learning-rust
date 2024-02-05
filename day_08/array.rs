//Arrays demo - Basic initialize and print

fn main() {
    //Declare integer array
    let num_array = [4, 7, -4, 0, 9];
    //Declare float array, specify type and size
    let fl_array:[f64;5] = [-7.4, 0.3, -5.12, 4.9, 8.3];
    //Declare string array, specify type and size
    let name_array:[&str;7] = ["Alice", "Bob", "Poonam", "Dexter", "Willow", "Ian", "Gera"];
    //Fill an integer array of size 3 with value 5
    let num_def_array:[isize;3] = [5;3];
    //Fill a float array of size 4 with value 0.0
    let fl_def_array:[f64;4] = [0.0;4];
    //Fille a string (&str) array of sizer 7 with value "NULL"
    let string_def_array:[&str;7] = ["NULL";7];
    //Fill a boolean array of size 5 with value true
    let boolean_def_array:[bool;4] = [true;4];
    //Print everything
    println!("\nnum_array: {:?}", num_array);
    println!("\nnum_array Size: {:?}", num_array.len());
    println!("\nfl_array: {:?}", fl_array);
    println!("\nfl_array Size: {:?}", fl_array.len());
    println!("\nname_array: {:?}", name_array);
    println!("\nname_array Size: {:?}", name_array.len());
    println!("\nnum_def_array: {:?}", num_def_array);
    println!("\nnum_def_array Size: {:?}", num_def_array.len());
    println!("\nfl_def_array: {:?}", fl_def_array);
    println!("\nfl_def_array Size: {:?}", fl_def_array.len());
    println!("\nstring_def_array: {:?}", string_def_array);
    println!("\nstring_def_array Size: {:?}", string_def_array.len());
    println!("\nboolean_def_array: {:?}", boolean_def_array);
    println!("\nboolean_def_array Size: {:?}", boolean_def_array.len());

}