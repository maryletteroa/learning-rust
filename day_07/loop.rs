// loop demo

fn main() {
    // Initialize counter to 1
    let mut count:isize = 1;
    // Iterate from 1 to 10
    loop {
        println!("{}", 5 * count);
        count += 1;
        if count == 11 {
            break;
        }
    }
}