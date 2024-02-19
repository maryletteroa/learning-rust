//Vector demo 
fn main(){
    //Declare integer vector
    let num_vector = vec![-5, 8, 0, -2, 9];
    //Declare float vector
    let fl_vector = vec![4.6, 0.5, 7.3];
    //Declare string vector
    let name_vector = vec!["Dexter", "Jerry", "Courage"];
    //Print everything
    println!("\nnum_vector: {:?}", num_vector);
    println!("\nnum_vector Size: {}", num_vector.len());
    println!("\nnum_vector: {:?}", fl_vector);
    println!("\nnum_vector Size: {}", fl_vector.len());
    println!("\nnum_vector: {:?}", name_vector);
    println!("\nnum_vector Size: {}", name_vector.len());
}