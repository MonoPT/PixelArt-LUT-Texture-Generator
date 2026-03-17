mod generate_lut;
mod apply_lut;

pub use apply_lut::apply_lut;

use generate_lut::{ImageRGBA, generate_lut};
use std::path::Path;

use anyhow::{Result};
use ril;

#[tokio::main]
async fn main() -> Result<()> { 
    
    /*apply_lut(
        std::path::Path::new("./assets/sample.png"),
        std::path::Path::new("./assets/lut_1.png"),
        std::path::Path::new("out_0.png")
    ).unwrap();*/
        
    let skins: Vec<ImageRGBA> = vec![
        ril::Image::open("./assets/a.png").unwrap(),
        ril::Image::open("./assets/b.png").unwrap(),
    ];
    
    generate_lut(skins, Path::new("./assets/sample.png")).await.unwrap();
    
    Ok(())
}



