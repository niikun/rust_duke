use std::io;
// #[derive(Debug)]

// fn print_str(s: &str){
//     println!("{}",s);
// }

// fn print_string(s:&String){
//     println!("{}",s);
// }

// fn main(){
//     let s1 = "Hello,Rust!";
//     let mut s2 = String::from("Hello,Rust!");
//     s2.push_str(" Welcome to Rust Programming!");
//     // s1.push_str(" Welcome to Rust Programming!");
    // print_str(&s1);
    // print_string(&s2);
    // print_str(&s2.as_str());
    // print_string(&s2);
// }
fn main(){
    let mut input = String::new();
    while input.trim() != "stop"{
        input.clear();
        println!("Please enter a word or 'stop' to exit:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}",input);

    }
    println!("Goodbye!");
}