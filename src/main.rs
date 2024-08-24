mod tag_label_maps;

use std::fs::File;
use std::io::BufReader;
use exif::{Exif, Field, In, Reader, Tag};
use clap::Parser;
use crate::tag_label_maps::TagLabelMaps;

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

    let maps = TagLabelMaps::get();

    println!("===== Camera =====\n");
    print_tag_map(maps.camera_tag_map, &exif);

    println!("\n===== Lens ===== \n");
    print_tag_map(maps.lens_tag_map, &exif);

    println!("\n===== Image =====\n");
    print_tag_map(maps.image_tag_map, &exif);

    println!("\n===== Other =====\n");
    print_tag_map(maps.other_tag_map, &exif);
}

fn print_tag_map(map: Vec<(Tag, &str)>, exif_reader: &Exif)
{
    for (tag, label) in map {
        if let Some(field) = exif_reader.get_field(tag, In::PRIMARY) {
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