/*This follows the tutorial course on w3schools website
*/ 

fn main() {
    introduction_and_variables();

    operators();

    conditions();

    loops();

    let mut user: &str = "john";
    print!("{} ", func_with_params(user));

    user = "eve";
    print!("{} ", func_with_params(user));

    user = "henry";
    print!("{} \n", func_with_params(user));

    working_with_strings();

    ownership_in_rust();

    references_and_ownership();
}

fn introduction_and_variables(){
    println!("Hello, world!");
    println!("Im running");

    //Same as println but doesnt insert a \n
    print!("HI ");
    print!("im behind hi\n");

    let first_var = "msg";
    println!("First variable holds: {}", first_var);
    
    let second_var = "duo";
    println!("First variable holds: {} | second variable holds: {}", first_var, second_var);

    /* #######################
    Variables cannot be CHANGED by default
    var x = 10;
    x = 5;
    --> error
    */

    //mut - mutable/changeable, allows changing of value
    let mut x = 10;
    println!("Before: {}", x);
    x = 5;
    println!("After: {}", x);

    let y = x + 10;
    x += y;
    println!("y: {}, x: {}", y, x);

    // data types
    let bool_var: bool = true;
    let neg_and_pos_int_var: i32 = -64;
    let only_zero_and_pos_int_var: u32 = 0; //u - unsigned
    let float_var: f64 = 6.39;
    let char_var: char = 'B';
    let string_var: &str = "IDK";
    println!("{}, {}, {}, {}, {}, {}", bool_var, neg_and_pos_int_var, only_zero_and_pos_int_var, float_var, char_var, string_var);

    //constants named in UPPER_CASE
    const YEAR: u32 = 2007;
    println!("{}", YEAR);
}

fn operators(){

    //Also +=, -=, *=, /=, %= could be used
    let add = 1 + 3;
    let sub = 4 - 3;
    let mul = 4 * 3;
    let div = 12 / 3;
    let rem = 14 % 3;

    println!("{}", add);
    println!("{}", sub);
    println!("{}", mul);
    println!("{}", div);
    println!("{}", rem);

    println!("COMPARISON OPERATORS");
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    println!("LOGICAL OPERATORS");
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin); //AND
    println!("Has any access: {}", logged_in || is_admin); //OR
    println!("Not logged in: {}", !logged_in); //NOT

    let age = 20;
    let can_vote = age >= 18;

    println!("Can vote? {}", can_vote); 

    if age >= 21 {
        println!("Can drink alcohol in the US");
    } else {
        println!("Cannot drink alcohol in the US");
    }

}

fn conditions(){
    let mut a = 5;
    if 1 > 0 {
        a -= 2;
        if  a > 3  {
            a = 0;
        }
        else if a > 2 {
            a = -1;
        }
        else {
            a = -2;
        }
    }

    // both  if else values must be the same type
    let b = if a == 0 { 20 } else { 10 };

    match b {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        6 => println!("6"),
        7 => println!("7"),
        8 => println!("8"),
        9 => println!("9"),
        10 => println!("10"),
        _ => println!("YAY"),
    }

    //Match multiple values
    let day = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    let return_value = match day {
        1 | 2 | 3 | 4 | 5 => "weekday",
        6 | 7 => "weekend",
        _ => "error",
    };
    println!("{}", return_value);
}

fn loops() {

    let mut i = 0;
    loop{
        if i == 3 {
            break;
        }
        print!("{} ", i);
        i += 1;
    };

    i = 0;
    print!("\n");
    let _return_value = loop{
        if i == 3 {
            break i;
        }
        print!("{} ", i);
        i += 1;
    };

    i = 0;
    print!("\n");
    while i < 5 {
        print!("{} ", i);
        i += 1;
    };

    i = 0;
    print!("\n");
    while i < 5 {
        if i == 2 {
            i += 1;
            continue;
        };
        print!("{} ", i);
        i += 1;
    };
    print!("\n");

    // non inclusive, goes only to 5
    for j in 1..6 {
        print!("{} ", j);
    };
    print!("\n");

    //include 6
    for j in 1..=6 {
        print!("{} ", j);
    };
    print!("\n");

}

fn func_with_params(name: &str) -> &str {
     return match name {
        "john" => "Welcome John",
        "eve" => "Welcome Eve",
        _ => "unknown user"
    };
}

fn working_with_strings(){
    /* 
    &str - string slices, unmodifiable
    String - changeable 
    */
    let a: &str  = "Red Wine";
    let mut txt = a.to_string();
    let mut txt2 = String::from(a);

    txt.push_str(" and cheese"); //adds string
    txt2.push('!'); // adds one char

    println!("{}", format!("{}, I love it!\n{}\n{}", txt, txt2, txt.len()));
}

fn ownership_in_rust(){
    let a = String::from("IDK HI");
    let b = a;
    //println!("{}", a); would throw error, a no longer owns the value
    println!("{}", b);

    //But simple types like numbers, characters and booleans are copied, not moved
    let c = 10;
    let d = c;
    println!("{} {}", c, d);

    //String can be cloned
    let e = String::from("RANDOM");
    let f = e.clone();
    println!("{} {}", e, f);
}

fn references_and_ownership(){
    let a = String::from("HELLO");
    let b = &a;
    println!("{} {}", a, b);
    
    let mut c = String::from("HELLO");
    let d = &mut c;
    d.push_str(" BYE");
    println!("{}", c);
}