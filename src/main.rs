use std::fs::File;
use std::io::BufReader;
use exif::{In, Reader, Tag};

fn main() {
    let file = match File::open("tests/DSCF0576.JPG") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        },
    };

    let exif = match Reader::new().read_from_container(&mut BufReader::new(file)) {
        Ok(exif) => exif,
        Err(err) => {
            eprintln!("Error reading exif data: {}", err);
            return;
        }
    };

    let map = vec![
        (Tag::LensMake, "Maker"),
        (Tag::LensModel, "Model"),
        (Tag::LensSpecification, "Specs"),
        (Tag::LensSerialNumber, "Serial Number"),
    ];

    println!("Lens:\n");

    for (tag, label) in map {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            let value: Vec<String> = field
                .display_value()
                .to_string()
                .replace("\"", "")// Cleanup all the quotes
                .split(',')
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect();

            println!("{}: {}", label, value.join(""))
        }
    }
}