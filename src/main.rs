use qrcode::QrCode;
use qrcode::render::svg;
use resvg::{usvg, tiny_skia, FitTo};
use resvg::usvg::TreeParsing;
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs;

fn generate_qr_code(text: &str, filename: &str, format: &str) {
    match QrCode::new(text) {
        Ok(code) => {
            let svg_data = code.render::<svg::Color>().build();
            
            if format == "svg" {
                let output_filename = format!("{}.svg", filename);
                let mut file = File::create(&output_filename).unwrap();
                file.write_all(svg_data.as_bytes()).unwrap();
                println!("QR Code saved as {}", output_filename);
            } else {
                let opt = usvg::Options::default();
                let tree = usvg::Tree::from_str(&svg_data, &opt).expect("Failed to parse SVG");
                
                let size = tree.view_box.rect.size();
                let mut pixmap = tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32)
                    .expect("Failed to create Pixmap");
                
                resvg::render(&tree, FitTo::Original, tiny_skia::Transform::default(), pixmap.as_mut())
                    .expect("Failed to render SVG");
                
                let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
                    size.width() as u32,
                    size.height() as u32,
                    pixmap.data().to_vec()
                ).expect("Failed to create ImageBuffer");
                
                let output_filename = format!("{}.png", filename);
                img.save(&output_filename).expect("Failed to save PNG file");
                println!("QR Code saved as {}", output_filename);
            }
        }
        Err(_) => println!("Failed to generate QR code."),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: {} <text|URL|file_path> [output_filename|-] [format: png|svg]", args[0]);
        std::process::exit(1);
    }
    
    let input = &args[1];
    let output_filename_arg = args.get(2).map(String::as_str).unwrap_or("qrcode");
    let output_filename =  if output_filename_arg == "-" {
        "qrcode"
    } else {
        output_filename_arg
    };
    let format = args.get(3).map(String::as_str).unwrap_or("png");
    let text = if fs::metadata(input).is_ok() {
        fs::read_to_string(input).unwrap_or_else(|_| {
            eprintln!("Error reading file: {}", input);
            std::process::exit(1);
        })
    } else {
        input.clone()
    };

    generate_qr_code(text.trim(), output_filename, format);
}
