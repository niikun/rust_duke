// enum Shape{
//     Circle(f64),
//     Square(f64),
// }

// enum FileSize{
//     Bytes(u64),
//     Kilobytes(u64),
//     Megabytes(u64),
//     Gigabytes(u64),
// }

enum WineRegions{
    Bordeaux,
    Burgundy,
    Champagne,
    Rhone,
    Alsace,
    Loire,
    Languedoc,
    Provence,
    Corsica,
    Jura,
    Savoie,
    SouthWest,
    Beaujolais,
    Other,
}

struct Wine{
    name: String,
    region: WineRegions,
}

// const PI: f64 = 3.14;

fn main() {
    // let circle = Shape::Circle(3.0);
    // let square = Shape::Square(4.0);
    // println!("Area of circle: {:.2}", area(circle));
    // println!("Area of square: {:.2}", area(square));
    // let file = FileSize::Bytes(1000);
    // print_size(file);
    let wine = Wine{
        name:String::from("niikun wine"),
        region:WineRegions::Bordeaux,
    };

    match wine.region{
        WineRegions::Bordeaux => println!("{} is Bordeaux",wine.name),
        WineRegions::Burgundy => println!("Burgundy"),
        WineRegions::Champagne => println!("Champagne"),
        WineRegions::Rhone => println!("Rhone"),
        WineRegions::Alsace => println!("Alsace"),
        WineRegions::Loire => println!("Loire"),
        WineRegions::Languedoc => println!("Languedoc"),
        WineRegions::Provence => println!("Provence"),
        WineRegions::Corsica => println!("Corsica"),
        WineRegions::Jura => println!("Jura"),
        WineRegions::Savoie => println!("Savoie"),
        WineRegions::SouthWest => println!("SouthWest"),
        WineRegions::Beaujolais => println!("Beaujolais"),
        WineRegions::Other => println!("Other"),
    }
}

// fn area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => {
//             PI * radius * radius
//         },
//         Shape::Square(side) => {
//             side * side
//         }
//     }
// }

// fn print_size(file: FileSize) {
//     match file {
//         FileSize::Bytes(u64) => println!("{} bytes", u64),
//         FileSize::Kilobytes(u64) => println!("{} KB", u64),
//         FileSize::Megabytes(u64) => println!("{} MB", u64),
//         FileSize::Gigabytes(u64) => println!("{} GB", u64),
//     }
// }
