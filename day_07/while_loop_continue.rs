//continue demo
fn main() {
    // Initialize counter to 1
    let mut count:isize = 1;
    let mut multiple;
    //Iterate from 1 to 10
    while count <= 10 {
        multiple = count * 3;
        if multiple % 5 == 0 { 
            count += 1;
            continue;
        }
        println!("{}", multiple);
        count += 1;
    }
}