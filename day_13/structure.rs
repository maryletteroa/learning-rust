//Structure demo 1
//Declare a structure
struct Point {
    x:i32,
    y:i32,
}

fn main() {
    //Initialize a structure variable
    let p1 = Point {
        x: 1,
        y: 2,
    };
    //Initialize another structure
    let p2 = Point {
        x: 6,
        y: 8,
    };
    //Print structure variables
    println!("\nPoint p1:\nx = {}\ty = {}", p1.x, p1.y);
    println!("\nPoint p2:\nx = {}\ty = {}", p2.x, p2.y);

}