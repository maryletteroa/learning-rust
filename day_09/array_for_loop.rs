//Arrays demo - for loop with arrays
fn main(){
    //Declare string array, specify type and size
    let city_array:[&str;5] = ["Taipe", "Casablanca", "Oslo", "Santiago", "Waterloo"];
    println!("\ncity_array printed using for loop:\n");
    for city in city_array.iter() {
        println!("\n{}", city);
    }
    println!("\n");
}