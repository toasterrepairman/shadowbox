extern crate image;

use image::{GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // Check for command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_image>", args[0]);
        std::process::exit(1);
    }

    // Load the input image
    let input_path = &args[1];
    let img = match image::open(input_path) {
        Ok(img) => img.to_rgb8(),
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    // Define parameters for the OBJ generation
    let width = img.width() as usize;
    let height = img.height() as usize;
    let scale = 0.1; // Adjust the scale factor as needed
    let base_height = 0.0; // Height of the baseplate
    let output_path = "output.obj";

    // Create and open the output OBJ file
    let mut obj_file = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            std::process::exit(1);
        }
    };

    // Write OBJ header
    writeln!(obj_file, "mtllib material.mtl").unwrap();

    // Define material for the plane
    writeln!(obj_file, "usemtl plane_material").unwrap();
    writeln!(obj_file, "newmtl plane_material").unwrap();
    writeln!(obj_file, "Kd 1.0 1.0 1.0").unwrap(); // White color
    writeln!(obj_file, "Ka 0.0 0.0 0.0").unwrap(); // Ambient color

    // Generate vertices and faces based on the grayscale image
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x as u32, y as u32);
            let grayscale_value = (pixel[0] as f32) / 255.0;
            let vertex_x = (x as f32) * scale;
            let vertex_y = (y as f32) * scale;
            let vertex_z = grayscale_value * scale + base_height;

            writeln!(obj_file, "v {} {} {}", vertex_x, vertex_y, vertex_z).unwrap();
        }
    }

    // Generate faces for the plane
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let vertex1 = y * width + x + 1;
            let vertex2 = y * width + x + 2;
            let vertex3 = (y + 1) * width + x + 2;
            let vertex4 = (y + 1) * width + x + 1;

            writeln!(obj_file, "f {} {} {} {}", vertex1, vertex2, vertex3, vertex4).unwrap();
        }
    }

    println!("OBJ file '{}' generated successfully.", output_path);
}
