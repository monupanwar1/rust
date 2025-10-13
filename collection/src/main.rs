

fn main() {
  let mut todos = Vec::new();
  todos.push("learn rust");
  todos.push("build a project");

  println!("MY todos");

  for todo in &todos{
    println!("{}",todo);
  };

  todos.pop();

  println!("after removing last");
  for todo in &todos{
      println!("{}",todo);
  }


}
