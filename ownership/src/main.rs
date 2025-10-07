struct User{
    name:String,
    age:u32,
}
fn print_name(u:&User){
    println!("name is {}",u.name);
}
fn increment_age(u:&mut User){
    u.age+=1;
    println!("age is {}",u.age);
}
// fn consume_user(u:User){
//     println!("user {} is now deleted from memory",u.name);
// }

fn main() {
   let mut user1= User{
    name:String::from("KunalPanwar"),
     age:20,
   };
   print_name(&user1);
   increment_age(&mut user1);
   
//    consume_user(user1);  //ownership moved here (user1 invalid now)

   
   let user2= &user1;
    
   println!("{}",user2.name)


}
