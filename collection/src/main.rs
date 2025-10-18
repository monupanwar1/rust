//vec
// fn main() {
//   let mut todos = Vec::new();
//   todos.push("learn rust");
//   todos.push("build a project");

//   println!("MY todos");

//   for todo in &todos{
//     println!("{}",todo);
//   };

//   todos.pop();

//   println!("after removing last");
//   for todo in &todos{
//       println!("{}",todo);
//   }

// }

// use std::collections::HashMap;

// fn main() {
//     let mut marks = HashMap::new();

//     marks.insert("Math", 90);
//     marks.insert("Science", 90);
//     marks.insert("Hindi", 90);
//     marks.insert("English", 93);

//     if let Some(mark) = marks.get("Science") {
//         println!("Marks:{}", mark)
//     };

//     marks.insert("Math", 100);
//     marks.remove("Hindi");

//     for(subject,marks) in &marks{
//       println!("{}=>{}",subject,marks);
//     }


// }

use std::collections::HashSet;

fn main(){
  let mut fruits =HashSet::new();
  fruits.insert("Apple");
  fruits.insert("Apple");
  fruits.insert("Banana");
  fruits.insert("Mango");

   println!("{:?}", fruits);

   if fruits.contains("Mango"){
    println!("Mango is in the set");
    
   }
   fruits.remove("Banana");

   for fruit in &fruits{
    println!("{}",fruit);
   }

}
