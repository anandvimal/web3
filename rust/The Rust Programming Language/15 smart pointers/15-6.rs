fn main() {
    let x = 5;
    let y = &x;  // y is a reference to x

    println!("x = {}, y = {}", x, y); // y is printed as a reference
    println!("y points to {}", *y); // dereferencing y to get the value 
    //Importantly, &i32 also implements Display via a blanket implementation in the standard library: impl<T> Display for &T where T: Display + ?Sized. This means when you format a reference like y with {}, it automatically delegates to the Display implementation of the referenced value (*y, which is i32).


    assert_eq!(5, x);
    //assert_eq!(5, y);  // Now compares i32 to &i32
    assert_eq!(5, *y); // works with dereferencing // Now compares i32 to i32

    }