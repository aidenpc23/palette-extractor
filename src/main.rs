use std::{env, path, process, fs};
use std::num::ParseIntError;

use color_thief::ColorFormat;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Error: wrong number of arguments. Expected 2, got {}", args.len() - 1);
        print_usage(&args[0]);
        process::exit(1);
    }
    
    let image_path = &args[1];
    let num_cols = parse_u8(&args[2]);
    
    if !fs::metadata(image_path).is_ok() {
        eprintln!("Error: failed to open file {}", image_path);
        process::exit(1);
    }
    
    let img = match image::open(&path::Path::new(image_path)) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: failed to open image file: {}", e);
            process::exit(1);
        }
    };
    
    let (buffer, color_type) = get_image_buffer(img);
    let colors = match color_thief::get_palette(&buffer, color_type, 10, num_cols) {
        Ok(colors) => colors,
        Err(e) => {
            eprintln!("Error: failed to get palette: {}", e);
            process::exit(1);
        }
    };

    for color in colors.iter() {
        println!("{}", format_hex(color.r, color.g, color.b));
    }
}

fn parse_u8(input: &str) -> u8 {
    match input.parse::<u8>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: second argument must be a valid u8");
            process::exit(1);
        }
    }
}

fn get_image_buffer(img: image::DynamicImage) -> (Vec<u8>, ColorFormat) {
    match img {
        image::DynamicImage::ImageRgb8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgb)
        }
        image::DynamicImage::ImageRgba8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgba)
        }
        _ => unreachable!(),
    }
}

fn format_hex(r: u8, g: u8, b: u8) -> String {
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

fn print_usage(program_name: &str) {
    println!("Usage: {} <image_path> <num_cols>", program_name);
    println!("  <image_path>: path to the image file");
    println!("  <num_cols>: number of colors in the palette (must be a valid u8)");
}