
// This function will not compile because the lifetimes of the references
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } 
//     else { y }
// }


// To fix it, we need to specify that the returned reference will have the same lifetime
// as the smaller of the lifetimes of the input references.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } 
    else { y }
}


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}