use std::collections::HashMap;

fn main() {
    
    let mut reviews: HashMap<String, String> = HashMap::new();

    // reviews.insert(String::from("Ancient Roman History"),
    //                 String::from("Very accurate."));
    // reviews.insert(String::from("Cooking with Rhubarb"),
    //                 String::from("Sweet recipes."));
    // reviews.insert(String::from("Programming in Rust"),
    //                 String::from("Great examples."));

    insert_reviews(&mut reviews,String::from("Ancient Roman History"),
                    String::from("Very accurate."));
    insert_reviews(&mut reviews,String::from("Cooking with Rhubarb"),
                    String::from("Sweet recipes."));
    insert_reviews(&mut reviews,String::from("Programming in Rust"),
                    String::from("Great examples."));
    print_reviews(&mut reviews,String::from("Ancient Roman History"));

    let obsolete = String::from("Cooking with Rhubarb");
    println!("\n'{}\' Removed.",&obsolete);

    reviews.remove(&obsolete);

    println!{"{:?}",&mut reviews};

}

fn insert_reviews(reviews:&mut HashMap<String, String>,book:String, review:String){
    reviews.insert(book, review);
}

fn print_reviews(reviews:&mut HashMap<String, String>,book:String){
    match reviews.get(&book){
        Some(review) => println!("{}: {}",book,review),
        None => println!("{} has no review",book),
    }
}
