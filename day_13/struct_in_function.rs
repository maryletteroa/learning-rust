//Structure demo 3 -- Passing structure variable to a function
//Declare a structure

struct Country {
    name:String,
    capital:String,
    continent:String,
    calling_code:i32
}

fn show_country(c:Country) {
    println!("\nCountry Name: {}",c.name);
    println!("\nCapital: {}",c.capital);
    println!("\nContinent: {}",c.continent);
    println!("\nDialing Code: +{}",c.calling_code)
}

fn main() {
    //Initializate a structure variables
    let c1 = Country {
        name:String::from("Australia"),
        capital:String::from("Canberra"),
        continent:String::from("Australia"),
        calling_code:61
    };
    let c2 = Country {
        name:String::from("UK"),
        capital:String::from("London"),
        continent:String::from("Europe"),
        calling_code:44
    };
    let c3 = Country {
        name:String::from("South Africa"),
        capital:String::from("Capte Town"),
        continent:String::from("Africa"),
        calling_code:27
    };
    let c4 = Country {
        name:String::from("Japan"),
        capital:String::from("Tokyo"),
        continent:String::from("Asia"),
        calling_code:81
    };
    let c5 = Country {
        name:String::from("Canada"),
        capital:String::from("Ottawa"),
        continent:String::from("North America"),
        calling_code:1
    };
    //Pass these structure variables to show_country function
    show_country(c1);
    show_country(c2);
    show_country(c3);
    show_country(c4);
    show_country(c5);

}
