//String demo -- split / tokenize
fn main() {
    let str1 = String::from("Russia is the largest country by area");
    println!("\nstr1: {}", str1);
    let str2 = String::from("Netherlands,Norway,Japan,USA,India,Singapore");
     
    println!("\nstr2: {}", str2);
    //Tokenize str1 using split_whitespace
    println!("\nTokenize str1 by whitespace:\n");
    for str_token in str1.split_whitespace() {
        println!("{}", str_token);
    }
    println!("\nTokenize str2 by comma:\n");
    //Tokenize str2 using split(",")
    for str_token in str2.split(",") {
        println!("{}", str_token);
    }
}