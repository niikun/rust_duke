// struct Person {
//     first: String,
//     middle: Option<String>,
//     last: String,
// }

// fn build_full_name(person: &Person) -> String {
//     let mut full_name = String::new();
//     full_name.push_str(&person.first);
//     full_name.push_str(" ");
//     if let Some(middle) = &person.middle{
//         full_name.push_str(middle);
//         full_name.push_str(" ");
//     }

//     // TODO: Implement the part of this function that handles the person's middle name.

//     full_name.push_str(&person.last);
//     full_name
// }

// fn main() {
//     let john = Person {
//         first: String::from("James"),
//         middle: Some(String::from("Oliver")),
//         last: String::from("Smith"),
//     };
//     assert_eq!(build_full_name(&john), "James Oliver Smith");

//     let alice = Person {
//         first: String::from("Alice"),
//         middle: None,
//         last: String::from("Stevens"),
//     };
//     assert_eq!(build_full_name(&alice), "Alice Stevens");

//     let bob = Person {
//         first: String::from("Robert"),
//         middle: Some(String::from("Murdock")),
//         last: String::from("Jones"),
//     };
//     assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
// }

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0).unwrap());
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}