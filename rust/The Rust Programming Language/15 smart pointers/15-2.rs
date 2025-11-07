//Note that this code won’t compile yet because the List type doesn’t have a known size,
enum List {
    Cons(i32, List),
    Nil,
}