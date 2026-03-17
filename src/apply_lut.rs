use std::path::Path;
use ril::Image;
use std::collections::{HashMap};
use ril;
use ril::Rgba;

pub fn apply_lut(source_path: &Path, lut_path: &Path, out_path: &Path) -> ril::Result<()> {
    let source: Image<Rgba> = Image::open(source_path).unwrap();
    let lut: Image<Rgba> = Image::open(lut_path).unwrap();

    let mut map = HashMap::new();
    for x in 0..lut.width() {
        let key = lut.get_pixel(x, 0); 
        let val = lut.get_pixel(x, 1);
        map.insert(key, val);
    }

    let output = source.map_pixels(|p| {
        map.get(&Some(&p)).copied().unwrap_or(Some(&p)).unwrap().to_owned()
    });

    output.save_inferred(out_path).unwrap();
    
    Ok(())
}