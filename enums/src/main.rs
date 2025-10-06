
enum AppState {
    Loading,
    Success(String),
    Error(String)
    
}
impl  AppState {
    fn show_message(&self){
        match  self {
            AppState::Loading => println!("⏳ Loading... please wait"),
            AppState::Success(data) => println!("✅ Success: {}", data),
            AppState::Error(msg) => println!("❌ Error: {}", msg),
            
        }
    }
    
}

fn main() {
    let state1=AppState::Loading;
    let state2=AppState::Success(String::from("Data loaded Successfully"));
    let state3=AppState::Error(String::from("Data loading Failed"));

    state1.show_message();
    state2.show_message();
    state3.show_message();
   
    
}
