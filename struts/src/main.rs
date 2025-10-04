struct Person{
    name:String,
    age :u32,
}
impl Person{
    fn greet(&self){
        println!("Hello ,my name is {} ",self.name);
    }
    fn birthday(&mut self){
        self.age +=1;
        println!("Happy birthday ,my age is {}",self.age);
    }
}

fn main(){
    let mut person=Person{
        name:String::from("kunal Panwar"),
        age:21,
    };
    person.greet();
    person.birthday();
}