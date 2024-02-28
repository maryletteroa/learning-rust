//Function demo - many function that accepts no arguments, does not return any values

//function1 definition
fn function1(){
    println!("\nInside function1.\n");
}

fn function2(){
    println!("\nInside function2.\n");
}

//main function
fn main(){
    println!("\nInside main function.\n");

    //Call function1
    function1();
    //Call function2
    function2();
    //Call function3
    function3();
}

//function3 definition
fn function3() {
    println!("\nInside function3.\n");
}