pub fn run(){
    let maybe_number = Some(42);
    // if let Some(number) = maybe_number{
    //     println!("number is {}", number);
    // } else {
    //     println!("number is not available");
    // }
    if let Some(number) = maybe_number{
        match number{
            None => println!("rnumber is not available"),
            Some(number) => println!("number is {}", number),
        }
    }
}