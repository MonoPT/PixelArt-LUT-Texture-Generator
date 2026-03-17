use std::collections::{HashMap};
use std::path::Path;
use anyhow::{Result};
use ril;

use std::collections::BTreeSet;
pub type ImageRGBA = ril::Image<ril::Rgba>;
type ColorMap = HashMap<ril::Rgba, Vec<ColorPos>>;

type ColorPos = (u32, u32);

pub async fn generate_lut(skins: Vec<ImageRGBA>, save_location: &Path) -> Result<()> {
    let mut maps = vec![];
    
    for skin in skins.iter() {
        let coords = map_color_to_coords(&skin).await;
        
        maps.push(coords);
    }

    let mut data = vec![];
    
    for skin in maps.iter() {
        for c in skin {                        
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
    
    
    create_ref_img(result, skins, save_location);
    
    
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

pub fn create_ref_img(data: Vec<Vec<&str>>, skins: Vec<ImageRGBA>, save_location: &Path) {
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
    
    image.save_inferred(save_location).unwrap();
}