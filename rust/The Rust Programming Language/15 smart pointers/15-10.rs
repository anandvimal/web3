//Note: There’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target= T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {

    x = 5;
    let y = MyBox::new(x); // y is a MyBox pointing to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // Now compares i32 to i32 // works with dereferencing as we implemented Deref
}

