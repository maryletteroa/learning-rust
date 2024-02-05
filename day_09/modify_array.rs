//Arrays demo - element modification 
fn main() {
    //Declare mutable float array
    let mut num_array_f:[f64;5] = [1.67,-8.45,3.76,-5.19,7.13];
    //print original array
    println!("num_array_f:{:?}", num_array_f);
    //Change the elements of num_array_f
    num_array_f[0] = 0.0;
    num_array_f[1] = 6.09;
    num_array_f[2] = 1.73;
    num_array_f[3] = 8.23;
    num_array_f[4] = 2.22;
    //print original array
    println!("\nnum_array_f (after modifications): {:?}", num_array_f);
}