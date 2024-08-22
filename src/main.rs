use std::fs::File;
use std::io::BufReader;
use exif::{Field, In, Reader, Tag};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long, help = "Path to the file to analyze")]
    filepath: String,
}

fn main() {
    let args = Args::parse();

    let file = match File::open(args.filepath) {
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

    let camera_tag_map = vec![
        (Tag::Make, "Maker"),
        (Tag::Model, "Model")
    ];

    let lens_tag_map = vec![
        (Tag::LensMake, "Maker"),
        (Tag::LensModel, "Model"),
        (Tag::LensSpecification, "Specs"),
        (Tag::LensSerialNumber, "Serial Number"),
    ];

    println!("Camera:\n");
    for (tag, label) in camera_tag_map {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", label, format_field(field))
        }
    }

    println!("\nLens:\n");

    for (tag, label) in lens_tag_map {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", label, format_field(field))
        }
    }
}

fn format_field(field: &Field) -> String
{
    let value_array: Vec<String> = field
        .display_value()
        .to_string()
        .replace("\"", "")// Cleanup all the quotes
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    value_array.join("")
}