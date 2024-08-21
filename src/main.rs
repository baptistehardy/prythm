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

    println!("Lens:\n");

    for tag in [Tag::LensMake, Tag::LensModel, Tag::LensSpecification, Tag::LensSerialNumber] {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", field.tag, field.display_value().with_unit(&exif))
        }
    }
}