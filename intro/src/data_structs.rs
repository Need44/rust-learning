//import hashmap
use std::collections::HashMap;

pub fn array_and_vector(){
    //not resizable, same-type
    let arr = ["foo", "bar", "moo"];
    println!("{}", arr[1]);

    //printing whole array - through loops or by writing {:?}
    println!("{:?}", arr);

    //resizable arr -> vec, same-type
    let mut vec = vec!["apple", "banana"];
    vec.push("orange");
    vec[0] = "strawberry";
    println!("{:?}", vec);
    vec.pop();
    println!("{:?}", vec);

    vec.insert(0, "pear");
    vec.remove(0);
    println!("{:?}", vec);

    // note the borrowing of vec, data is not moved so it can be used again
    for fruit in &vec {
        print!("{} ", fruit);
    }
    print!("\n");
}

fn get_info() -> (String, i32, u32){
    //returns weather - desc, temp, wind speed
    (String::from("Sunny"), 17, 4)
}

pub fn tuples_and_hashmaps(){
    let a = ("Tiger", 8, true);
    println!("{} {} {}", a.0, a.1, a.2);

    let (animal, count, is_in_habitat) = a;
    println!("Animal: {}, In habitat?: {}, Count: {}", animal, is_in_habitat, count);

    println!("{:?}", get_info());
    let tmp = get_info();
    println!("{}, temperature: {}° C, wind: {} m/s", tmp.0, tmp.1, tmp.2);

    let mut capital_cities = HashMap::new();

    capital_cities.insert("Slovakia", "Bratislava");
    capital_cities.insert("Czechia", "Prague");

    println!("Capital of Czech republic is: {}", capital_cities["Czechia"]);

    capital_cities.insert("Slovakia", "Nitra");
    capital_cities.remove("Slovakia");

    for (country, capital) in &capital_cities{
        println!("{} {}", country, capital);
    }

}

struct Car {
    brand: String,
    model: String,
    year: u32,
}

pub fn custom_structs(){
    let car = Car {
        brand: String::from("Skoda"),
        model: String::from("Superb"),
        year: 2020,
    };
    println!("{}", car.model);
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn trying_enums(){
    let dir = Direction::Left;

    let mut result = match dir {
        Direction::Up => "UP",
        Direction::Down => "DOWN",
        Direction::Left => "LEFT",
        Direction::Right => "RIGHT",
    };
    
    println!("{}", result);
}