fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y
    println!("x = {}", x); // Prints x = 6
    
    let z = &x; // z is an immutable reference to x
    println!("x = {}", z);//This is ok,because z is immutable
}