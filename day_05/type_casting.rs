fn main() {
    //Declare some variables
    let a:isize = 24;
    let b:isize =5;
    //Type cast a and b as f64 and then divide
    let q:f64 = (a as f64) / (b as f64);
    //Print everything
    println!("\na = {} \nb = {}\nq = {}\n", a, b, q);
}