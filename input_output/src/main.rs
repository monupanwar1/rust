use std::io;

fn main() {
    let mut _todo1 = String::new();
    let mut _todo2 = String::new();

    println!("simple todo");

    println!("enter first todo");

    //taking input
    io::stdin()
        .read_line(&mut _todo1)
        .expect("failed to read line");
    let todo1 = _todo1.trim();
    //taking second input

    println!("enter second todo");
    io::stdin()
        .read_line(&mut _todo2)
        .expect("failed to read line");
    let todo2 = _todo2.trim();

    println!("list of todo");
    println!("1:{}", todo1);
    println!("2:{}", todo2);
}
