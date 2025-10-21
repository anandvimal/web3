fn main() {
    let x = 5;
    let y = &x;  // y is a reference to x

    println!("x = {}, y = {}", x, y); // y is printed as a reference
    println!("y points to {}", *y); // dereferencing y to get the value


    assert_eq!(5, x);
    //assert_eq!(5, y); // fails without dereferencing
    }