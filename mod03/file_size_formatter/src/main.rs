#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kirobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

use std::env;


fn format_size(size :u64) -> String{
    let filesize = match size{
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kirobytes(size as f64/1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64/1_000_000.0),
        _ => FileSize::Gigabytes(size as f64/1000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(size) => format!("{} bytes",size),
        FileSize::Kirobytes(kb) => format!("{:.2} KB",kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB",mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB",gb),
    }
}


fn main() {
    let args:Vec<String> = env::args().collect();
    println!("My Path is {}",args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
