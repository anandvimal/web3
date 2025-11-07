struct Point<T> {
    x: T,
    y: T,
}

struct MixPoint <T, U> {
    x: T,
    y: U,
}

fn main(){
    let _integer = Point {x : 5, y : 10};
    let _float = Point {x : 1.0, y : 4.0};

    let both_integer = MixPoint { x: 1, y:2};
    let both_float = MixPoint { x: 1.0, y:2.0};

    let integer_and_float = MixPoint{x:5, y: 4.0};
    let float_and_integer = MixPoint{x:1.1, y:2};
}