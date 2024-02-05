//Arrays demo - slice and loops
fn main () {
    //Declare integer array
    let num_array = [5, 9, 1, 0, 4, 2, 1, 8];
    //Declare float array, specify type and size
    let fl_array:[f64;4] = [11.2, 87.4, 91.6, 32.5];
    //Declare string array, specify type and size
    let country_array:[&str;5] = ["India", "Greece", "Thailand", "Armenia", "Ukraine"];
    //Create num_array slice from 2 to 6
    let num_slice = &num_array[2..6];
    //Create fl_array slice from 1..3
    let fl_slice = &fl_array[1..3];
    //Create country_array slice from 0..4
    let country_slice = &country_array[0..4];
    //Print everything
    println!("\nnum_array: {:?} \nnum_array Size: {}", num_array, num_array.len());
    println!("\nfl_array: {:?} \nfl_array Size: {}", fl_array, fl_array.len());
    println!("\ncountry_array: {:?} \ncountry_array Size: {}", country_array, country_array.len());
    println!("\nnum_slice: {:?} \nnum_slice Size: {}", num_slice, num_slice.len());
    println!("\nfl_slice: {:?} \nfl_slice Size: {}", fl_slice, fl_slice.len());
    println!("\ncountry_slice: {:?} \ncountry_slice Size: {}", country_slice, country_slice.len());
}