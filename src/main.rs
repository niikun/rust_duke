use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    insert_reviews(&mut reviews, String::from("Ancient Roman History"), String::from("Very accurate."));
    insert_reviews(&mut reviews, String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    // ...existing code...
}

fn insert_reviews(reviews: &mut HashMap<String, String>, book: String, review: String) {
    reviews.insert(book, review);
    // ...existing code...
}