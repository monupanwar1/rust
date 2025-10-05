fn point<T: std::fmt::Debug>(value: T) {
    println!("value:{:?}", value)
}


struct Point<T:std::fmt::Display>{
    x:T,
    y:T
}
#[derive(Debug)]
struct Point1<X,Y>{
    x:X,
    y:Y
}

fn main() {
    let int_point=Point{x:5,y:10};
    println!("{:?},{:?}",int_point.x,int_point.y);
    let int_point1=Point1{x:5,y:String::from("hello")};
    println!("{:?},{:?}",int_point1.x,int_point1.y);
    point(5);
}
