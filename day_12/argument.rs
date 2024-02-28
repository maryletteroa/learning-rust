//Function demo - a function that accepts 1 argument, does not return any value

//main function
fn main() {
    //Declare an integer variable a
    let a:isize = 21;
    println!("\nInside main function.\na = {}", a);
    //Call show_triple, pass argument
    show_triple(a);
}

fn show_triple(a:isize) {
    println!("\nInside show_triple function.\na = {}, (a x 3) = {}", a, (a*3));
}