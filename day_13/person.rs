//Structure demo 2
//Declare a structure
struct Person {
    name:String,
    age:i32,
    gender:char,
    country:String,
}

fn main() {
    //Initialize a structure variable
    let p1 = Person {
        name:String::from("Fiona"),
        age:29,
        gender:'F',
        country:String::from("Brazil")
    };
    let p2 = Person {
        name:String::from("Stuart"),
        age:33,
        gender:'M',
        country:String::from("UK")
    };
    let p3 = Person {
        name:String::from("Lily"),
        age:27,
        gender:'F',
        country:String::from("Ireland")
    };
    //Print structure variables
    println!("\nPerson p1\nname: {}\nage: {}\ngender: {}\ncountry: {}\n",p1.name, p1.age, p1.gender, p1.country);
    println!("\nPerson p2\nname: {}\nage: {}\ngender: {}\ncountry: {}\n",p2.name, p2.age, p2.gender, p2.country);
    println!("\nPerson p2\nname: {}\nage: {}\ngender: {}\ncountry: {}\n",p3.name, p3.age, p3.gender, p3.country);
}