//Vector demo - for loop iteration
fn main() {
    //Declare float and int vector
    let float_vector = vec![3.59, -6.25, -0.23, 5.69, 8.42, 1.23];
    let int_vector = vec![0, 10, 20];
    println!("\nfloat_vedtor printed using while loop: \n");
    let mut i = 0;
    while i < float_vector.len() {
        println!("{}", float_vector[i]);
        i = i + 1;
    }
    println!("\nint_vector printed using for loop: \n");
    for num in int_vector {
        println!("{}", num);
    }
}