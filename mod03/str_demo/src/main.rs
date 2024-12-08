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
    let s1 = "the quick brown fox jumps over the lazy dog";
    println!("{}",&s1[0..=4]);

    let s2 = format!("this is a string:\n{}",s1);
    println!("{}",s2);

    // for s in s1.split_whitespace(){
    //     print!("{} ",s);
    // }

    let vec1:Vec<&str> = s1.split_whitespace().collect();
    println!("{:?}",vec1);

    for c in s1.chars(){
        match c{
            'a'|'i'|'u'|'e'|'o'=>print!("{}",c),
            _ => print!("."),
        }
    }
}