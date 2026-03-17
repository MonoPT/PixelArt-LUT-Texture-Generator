use std::collections::{HashMap};
use anyhow::{Result};
use ril;
use ril::Rgba;

use std::collections::BTreeSet;
type ImageRGBA = ril::Image<ril::Rgba>;
type ColorMap = HashMap<ril::Rgba, Vec<ColorPos>>;

type ColorPos = (u32, u32);

#[tokio::main]
async fn main() -> Result<()> { 
    
    apply_lut(
        std::path::Path::new("./assets/sample.png"),
        std::path::Path::new("./assets/lut_1.png"),
        std::path::Path::new("out_0.png")
    ).unwrap();
    
    return Ok(());
    
    let skins: Vec<ImageRGBA> = vec![
        ril::Image::open("./assets/a.png").unwrap(),
        ril::Image::open("./assets/b.png").unwrap(),
    ];
    
    let mut maps = vec![];
    
    for skin in skins.iter() {
        let coords = map_color_to_coords(&skin).await;
        
        maps.push(coords);
    }

    let mut data = vec![];
    
    for (i, skin) in maps.iter().enumerate() {
        for c in skin {
            let color = c.0;
            
            let color = format!("{}_{}_{}_{}", color.r, color.g, color.b, i);
            
            let mut pixels = c.1.iter().map(|c| format!("{}_{}", c.0, c.1)).collect::<Vec<_>>();
            
            pixels.sort();
            
            data.push(pixels);
        }
    }
    
    let unique_vecs: Vec<BTreeSet<String>> = data
            .into_iter()
            .map(|v| v.into_iter().collect::<BTreeSet<String>>())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
    
    let mut distribution_map: HashMap<Vec<usize>, Vec<&str>> = HashMap::new();
    
    // Get all unique strings across the whole dataset
    let mut all_unique_strings: BTreeSet<&str> = BTreeSet::new();
    for set in &unique_vecs {
        for item in set {
            all_unique_strings.insert(item);
        }
    }

    for item in all_unique_strings {
        let mut indices = Vec::new();
        for (idx, set) in unique_vecs.iter().enumerate() {
            if set.contains(item) {
                indices.push(idx);
            }
        }
        // Group strings that have the exact same index distribution
        distribution_map.entry(indices).or_default().push(item);
    }

    // 3. Collect the grouped vectors
    let mut result: Vec<Vec<&str>> = distribution_map
        .into_values()
        .collect();

    // Sort for readable output
    result.sort_by_key(|b| b.len());
    

    println!("Color groups: {}", result.len());
    
    create_ref_img(result, skins);
    
    Ok(())
}

pub async fn map_color_to_coords(skin: &ImageRGBA) -> ColorMap {
    let mut colors: ColorMap = HashMap::new();
    
    for x in 0..skin.width() {
        for y in 0..skin.height() {
            
            let color = skin.get_pixel(x, y).unwrap();
            
            if color.a < 1 {continue;}
            
            let pos: ColorPos = (x, y);
            
            if colors.contains_key(&color) {
                let c = colors.get_mut(&color).unwrap();
                
                c.push(pos);
            } else {
                colors.insert(color.to_owned(), vec![pos]);
            }
            
        }
    }
    
    colors
}

pub fn random_color() -> ril::Rgba {
    ril::Rgba::new(rand::random_range(0..255), rand::random_range(0..255), rand::random_range(0..255), 255)
}

pub fn create_ref_img(data: Vec<Vec<&str>>, skins: Vec<ImageRGBA>) {
    let skin_ref = skins.get(0).unwrap();
    
    let mut image = ril::Image::new(skin_ref.width(), skin_ref.height(), ril::Rgba::transparent());
    
    struct ColorHist {
        color: ril::Rgba,
        ref_point: (u32, u32)
    }
    
    let mut colors_hist: Vec<ColorHist> = vec![];
    
    for group in data.iter() {
        let mut col;
            
        loop {
            col = random_color();
            
            if !colors_hist.iter().find(|cl| {
               cl.color == col 
            }).is_some() {
                break;
            }
        }
        
        for (i, coord) in group.iter().enumerate() {
            let coord = coord.split("_").collect::<Vec<_>>().iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
            
            let x = coord.get(0).unwrap().to_owned();
            let y = coord.get(1).unwrap().to_owned();
            
            if i == 0 {
                colors_hist.push(ColorHist { color: col, ref_point: (x,y) });
            }
            
            image.set_pixel(x, y, col);
        }
    }
    
    // Create swapp pallets
    
    for (idx, skin) in skins.iter().enumerate() {
        let mut swap_img = ril::Image::new(data.len() as u32, 2, ril::Rgba::transparent());
        
        
        for (idx_xl, color) in colors_hist.iter().enumerate() {
            let coords = color.ref_point;
            let color = color.color;
            
            let real_col = skin.get_pixel(coords.0, coords.1).unwrap();
            
            swap_img.set_pixel(idx_xl as u32, 0, color);
            swap_img.set_pixel(idx_xl as u32, 1, real_col.to_owned());
        }
        
        swap_img.save_inferred(format!("./assets/lut_{idx}.png")).unwrap();
    }
    
    image.save_inferred("./assets/sample.png").unwrap();
}

use std::path::Path;
use ril::Image;

fn apply_lut(source_path: &Path, lut_path: &Path, out_path: &Path) -> ril::Result<()> {
    // 1. Load images - ensure we are working with Rgba specifically
    let source: Image<Rgba> = Image::open(source_path).unwrap();
    let lut: Image<Rgba> = Image::open(lut_path).unwrap();

    // 2. Build the map
    let mut map = HashMap::new();
    for x in 0..lut.width() {
        // We dereference (*) the pixels here so the HashMap stores values, not refs
        let key = lut.get_pixel(x, 0); 
        let val = lut.get_pixel(x, 1);
        map.insert(key, val);
    }

    // 3. Map pixels
    // 'p' is passed by value (Rgba) in ril's map_pixels
    let output = source.map_pixels(|p| {
        // .get(&p) returns Option<&Rgba>
        // .copied() turns it into Option<Rgba>
        // .unwrap_or(p) ensures we return a plain Rgba
        let c = map.get(&Some(&p)).copied().unwrap_or(Some(&p)).unwrap();
        
        c.to_owned()
    });

    // 4. Save
    output.save_inferred(out_path).unwrap();
    
    Ok(())
}