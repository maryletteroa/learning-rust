//Function demo - a function that accept 2 argument, returns their sum

fn add_two_numbers(a: isize, b:isize) -> isize {
    //Add a and b, store in sum
    let sum = a + b;
    //Return sum to the calling function
    return sum;
}

//main function
fn main() {
    //Declare two variables
    let x:isize = 34;
    let y:isize = 86;
    //Call add_two_numbers, pass x and y as arguments, receive sum in s
    let s = add_two_numbers(x, y);
    println!("\nx = {} y = {} sum = {}\n", x ,y ,s);
}