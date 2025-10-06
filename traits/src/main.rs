// define traits
trait Greet {
    fn say_hello(&self);
}
// define struct
struct Person {
    name: String,
}
// impl traits for stuct

impl Greet for Person {
    fn say_hello(&self) {
        println!("hello my name is {}", self.name);
    }
}

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Dog says: Woof! 🐶");
    }
}
impl Speak for Cat {
    fn speak(&self) {
        println!("Cat says: Meow! 🐱");
    }
}

fn main() {
    let p = Person {
        name: String::from("kunal"),
    };
    p.say_hello();

    let d = Dog;
    let c = Cat;

    d.speak();
    c.speak();
}
