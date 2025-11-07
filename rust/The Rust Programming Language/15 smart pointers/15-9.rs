//Note: There’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.


struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x); // y is a MyBox pointing to x

    println!("x = {}, y = {}", x, y.0); // Accessing the value inside MyBox directly
    println!("y points to {}", *y); // dereferencing y to get the value 

    assert_eq!(5, x);
    assert_eq!(5, *y); //error: could not compile `deref-example`

}

