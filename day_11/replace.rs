//String demo -- replace
fn main() {
    let str1 = String::from("What they said is, together they can win!");
    println!("\nstr1: {}", str1);
    //Replace they with we
    let str2 = str1.replace("they", "we");
    println!("\nstr2: {}", str2);
}