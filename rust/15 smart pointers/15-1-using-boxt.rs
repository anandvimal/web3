fn main(){
    let b = Box::new(5);
    println!("b = {b}");

    let c = Box::new("hello");
    println!("c = {c}");

    let d = Box::new(3.14);
    println!("d = {d}");

    
    let e = Box::new(vec![1, 2, 3]);
    // println!("e = {e}"); //this is not allowed because Vec does not implement Display?
    println!("e = {:?}", e); //using debug format instead

}