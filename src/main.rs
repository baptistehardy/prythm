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
        (Tag::Model, "Model"),
        (Tag::Software, "Software"),
        (Tag::BodySerialNumber, "Serial Number"),
        (Tag::CameraOwnerName, "Owner Name"),
    ];

    let lens_tag_map = vec![
        (Tag::LensMake, "Maker"),
        (Tag::LensModel, "Model"),
        (Tag::LensSpecification, "Specs"),
        (Tag::LensSerialNumber, "Serial Number"),
    ];

    let image_tag_map = vec![
        (Tag::ExposureTime, "Exposure Time"),
        (Tag::ApertureValue, "Aperture"),
        (Tag::BrightnessValue, "Brightness"),
        (Tag::Contrast, "Contrast"),
        (Tag::FocalLength, "Focal Length"),
    ];

    let other_tag_map = vec![
        (Tag::Acceleration, "Acceleration"),
        (Tag::Artist, "Artist"),
        (Tag::BitsPerSample, "Bits/sample"),
        (Tag::CameraElevationAngle, "Elevation Angle"),
        (Tag::CFAPattern, "CFA Pattern"),
        (Tag::ColorSpace, "Color Space"),
        (Tag::ComponentsConfiguration, "Components Configuration"),
        (Tag::CompressedBitsPerPixel, "Compressed Bits/Pixel"),
        (Tag::CustomRendered, "Custom Rendered"),
        (Tag::DateTime, "Date & time"),
        (Tag::DateTimeOriginal, "Date Time Original"),
        (Tag::DateTimeDigitized, "Date Time Digitized"),
        (Tag::DeviceSettingDescription, "Device Setting Description"),
        (Tag::DigitalZoomRatio, "Digital Zoom Ratio"),
        (Tag::ExifVersion, "EXIF Version"),
        (Tag::FileSource, "File Source"),
        (Tag::Flash, "Flash"),
        (Tag::FlashEnergy, "Flash Energy"),
        (Tag::FocalPlaneXResolution, "Focal Plane X Resolution"),
        (Tag::FocalPlaneYResolution, "Focal Plane Y Resolution"),
        (Tag::GainControl, "Gain Control"),
        (Tag::GPSLatitude, "GPS Latitude"),
        (Tag::GPSLongitude, "GPS Longitude"),
        (Tag::GPSAltitude, "GPS Altitude"),
        (Tag::ImageDescription, "Image Description"),
        (Tag::InteroperabilityIndex, "Interoperability Index"),
        (Tag::ISOSpeed, "ISO Speed Ratings"),
        (Tag::ISOSpeedLatitudeyyy, "ISO Speed Latitude Y"),
        (Tag::ISOSpeedLatitudezzz, "ISO Speed Latitude Z"),
        (Tag::LightSource, "Light Source"),
        (Tag::MeteringMode, "Metering Mode"),
        (Tag::Orientation, "Orientation"),
        (Tag::PhotometricInterpretation, "Photometric Interpretation"),
        (Tag::SpatialFrequencyResponse, "Spatial Frequency Response"),
        (Tag::SubjectArea, "Subject Area"),
        (Tag::GPSDOP, "GPS DOP"),
        (Tag::GPSSpeed, "GPS Speed"),
        (Tag::Compression, "Compression"),
        (Tag::ImageWidth, "Image Width"),
        (Tag::ImageLength, "Image Length"),
        (Tag::SamplesPerPixel, "Samples per Pixel"),
        (Tag::PlanarConfiguration, "Planar Configuration"),
        (Tag::YCbCrSubSampling, "Chrominance Subsampling"),
        (Tag::XResolution, "X Resolution"),
        (Tag::YResolution, "Y Resolution"),
        (Tag::ResolutionUnit, "Resolution Unit"),
        (Tag::StripOffsets, "Strip Offsets"),
        (Tag::RowsPerStrip, "Rows per Strip"),
        (Tag::StripByteCounts, "Strip Byte Counts"),
        (Tag::JPEGInterchangeFormat, "JPEG Interchange Format"),
        (Tag::JPEGInterchangeFormatLength, "JPEG Interchange Format Length"),
        (Tag::TransferFunction, "Transfer Function"),
        (Tag::WhitePoint, "White Point"),
        (Tag::PrimaryChromaticities, "Primary Chromaticities"),
        (Tag::YCbCrCoefficients, "Chrominance Coefficients"),
        (Tag::YCbCrPositioning, "Chrominance Positioning"),
        (Tag::ReferenceBlackWhite, "Black/White References"),
    ];

    println!("===== Camera =====\n");
    for (tag, label) in camera_tag_map {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", label, format_field(field))
        }
    }

    println!("\n===== Lens ===== \n");

    for (tag, label) in lens_tag_map {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", label, format_field(field))
        }
    }

    println!("\n===== Image =====\n");

    for (tag, label) in image_tag_map {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", label, format_field(field))
        }
    }

    println!("\n===== Other =====\n");

    for (tag, label) in other_tag_map {
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