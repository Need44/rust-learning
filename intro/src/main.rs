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
}
