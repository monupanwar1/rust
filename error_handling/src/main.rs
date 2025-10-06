fn main() {
    let result = divide(10, 0);
    let result1 = divide(10, 10);

    match result {
        Ok(value) => println!("Division Done:{value}"),
        Err(err) => println!("Oops! Error: {err}"),
    }
    match result1 {
        Ok(value) => println!("Division Done:{value}"),
        Err(err) => println!("Oops! Error: {err}"),
    }
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("cannot divide  by zero"));
    }
    Ok(x / y)
}
