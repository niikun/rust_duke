fn ownership(){
    let numbers = vec![1,2,3,4,5];
    let slice = &numbers[0..3];
    println!("{:?}",slice);
    println!("{:?}",numbers);
}

fn modifiable(){
    let mut numbers = vec![1,2,3,4,5];
    let slice = &mut numbers[0..3];
    slice[0] = 100;
    println!("{:?}",slice);
    println!("{:?}",numbers);
}

fn main() {
    ownership();
    modifiable();
}
