fn main() {
    //Immutability in Rust is important to grasp before we continue to project
    //By default, every variable in Rust in immutable

    //That's why we have a mut keyword
    let mut y = 5;
    println!("Before: {y}");
    y = y * 10;
    println!("After: {y}");

    //Shadowing is ability to use the same varible name, 
    //but have the capability to change the data types
    //By using let, we are creating another variables with the same name
    let x = 5;
    println!("Before shadowing: {x}");
    let x = 5 * 5;
    println!("After shadowing: {x}");
    let x = "x is a string";
    println!("After shadowing by changing the data type: {x}");

    //Both mutability and shadowing can happened at the same time
    //It's just a new variable that is mutable
    let mut z = "z is a string";
    println!("Before: {z}");
    let mut z = 5 * 5;
    println!("After: {z}");
    z = z - 2;
    println!("After shadowing: {z}");

}
