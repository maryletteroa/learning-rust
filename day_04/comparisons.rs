fn main() {
    //Declare and initialize some variables
    let a = 10;
    let b = 60;
    let c:f64;
    let d:f64;
    c = 7.89;
    d = -4.42;
    //Print a b c d
    println!("\na = {} b = {}\nc = {} d = {}", a, b, c, d);
    println!("\na > b: {}", (a > b));
    println!("\nd < c: {}", (d < c));
    println!("\na == b: {}", (a == b));
    println!("\nc != d: {}", (c != d));
    println!("\na <= b: {}", (a <= b));
    println!("\nd >= c: {}", (d >= c));
}