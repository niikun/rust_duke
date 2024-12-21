enum Shapes{
    Circle(f64),
    Square(f64),
}


fn main() {
    let shapes = vec![Shapes::Circle(5.0),Shapes::Square(3.0)];
    let mut total_area = 0.0;
    for shape in shapes{
        match shape{
            Shapes::Circle(radius) => total_area += std::f64::consts::PI * radius * radius,
            Shapes::Square(side)=> total_area += side * side,
        }
    }
    println!("Total area: {:.2}",total_area);
    
}
