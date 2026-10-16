/*This follows the tutorial course on w3schools website
*/ 

fn main() {
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

    operators();
    conditions();
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
    let mut b = if a == 0 { 20 } else { 10 };

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