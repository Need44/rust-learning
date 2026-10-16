//import hashmap
use std::collections::HashMap;

pub fn array_and_vector(){
    //not resizable, same-type
    let arr = ["foo", "bar", "moo"];
    println!("{}", arr[1]);

    //resizable arr -> vec, same-type
    let mut vec = vec!["apple", "banana"];
    vec.push("orange");

}

pub fn tuples_and_hashmaps(){
    let a = ("Tiger", 8, true);
    println!("{} {} {}", a.0, a.1, a.2);

    let mut capital_cities = HashMap::new();
    capital_cities.insert("Slovakia", "Bratislava");
    capital_cities.insert("Czechia", "Prague");
    println!("Capital of Czech republic is: {}", capital_cities["Czechia"]);

}