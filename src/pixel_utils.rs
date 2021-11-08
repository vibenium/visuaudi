use image::open;

pub fn img_to_vec(path: String) -> Result<Vec<u16>, &'static str> {

    let file = match image::open(path) {
        Ok(r) => Some(r),
        Err(_) => None,
    };

    if file.is_none() {
        return Err("Failure 1");
    } else {
        let rgb = file.unwrap().into_rgba();
        let pixels = rgb.pixels();

        let mut avg_pixels: Vec<u16> = Vec::with_capacity(pixels.len());
        for px in pixels {
            let r: u16 = px[0] as u16;
            let g: u16 = px[1] as u16;
            let b: u16 = px[1] as u16;
            avg_pixels.push((r + g + b) / 3);
        }

        println!("{}", avg_pixels.len());
        return Ok(avg_pixels);
    }
}

