mod generate_lut;
mod apply_lut;

pub use apply_lut::apply_lut;
use clap::{Parser, ArgGroup};

use generate_lut::{generate_lut};
use std::path::Path;

use anyhow::{Result};
use ril;

use image::ImageReader;


#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["input", "base"]),
))]
struct Args {
    // Pair 1
    #[arg(long)]
    input: Option<String>,
    
    #[arg(long)]
    output: Option<String>,

    // Pair 2
    #[arg(long)]
    base: Option<String>,
    
    #[arg(long)]
    lut_folder: Option<String>,
    
    #[arg(long)]
    out_folder: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {   
    let args = Args::parse();
    
    match (&args.input, &args.output, &args.base, &args.lut_folder, &args.out_folder) {
        (Some(input), Some(output), None, None, None) => {
            println!("Will convert input data to LUT");
            
            let skins = load_images_from_folder(Path::new(input)).unwrap();
            
            if skins.len() < 1 {
                panic!("Could not find any image to use");
            }
                        
            generate_lut(skins, Path::new(output)).await.unwrap();
        }
        (None, None, Some(base), Some(lut_folder), Some(out_folder)) => {
            println!("Will generate images from LUT and reference");
            
            let skins = load_images_from_folder(Path::new(lut_folder)).unwrap();
            
            let base = load_image(Path::new(&base)).unwrap();
            
            if skins.len() < 1 {
                panic!("Could not find any image to use");
            }
            
            let out_path = Path::new(&out_folder);
            
            if !out_path.exists() {
                panic!("Could not find output folder");
            }
            
            for (idx, skin) in skins.iter().enumerate() {
                let t = format!("{out_folder}/output_{}.png", idx);
                let out_file = Path::new(&t);
                
                apply_lut(base.clone(), skin.to_owned(), out_file).unwrap();
            }
        }
        _ => {
            eprintln!("Invalid argument combination. Provide either input/output or base/lut_folder/.");
        }
    }
    
    Ok(())
}

// Stuff releated with loading and fixing invalid images

fn is_image(path: &Path) -> bool {
    match ImageReader::open(path) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(reader) => reader.decode().is_ok(),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

fn load_images_from_folder(path: &Path) -> Result<Vec<ril::Image<ril::Rgba>>, Box<dyn std::error::Error>> {
    let mut images = vec![];
    
    if path.exists() && path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
    
            if path.is_file() && is_image(&path) {
                images.push(load_image(Path::new(&path)).unwrap());
            }
        }
        
        return Ok(images);
    }
    
    panic!("Input must be a folder!");
}

fn load_image(path: &Path) -> Result<ril::Image<ril::Rgba>, Box<dyn std::error::Error>> { 
    use image::ImageReader;
    use image::ImageFormat::Png;
    use std::io::Cursor;
    use ril::{Image, Rgba};
    
    
    let img = ImageReader::open(path)?
            .decode()?;
    
    let mut buf = Vec::new();

    // Re-encode as standard RGBA PNG
    img.write_to(&mut Cursor::new(&mut buf), Png)?;

    let ril_img: Image<Rgba> =
        Image::from_bytes(ril::ImageFormat::Png, &buf)?;
        
    
    Ok(ril_img)
}
