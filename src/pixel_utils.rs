use image::open;

pub fn img_to_vec(path: String) -> Result<Vec<f32>, &'static str> {

    let file = match image::open(path) {
        Ok(r) => Some(r),
        Err(_) => None,
    };

    if file.is_none() {
        return Err("Failure 1");
    } else {
        let rgb = file.unwrap().into_rgba();
        let pixels = rgb.pixels();

        let mut avg_pixels: Vec<f32> = Vec::with_capacity(pixels.len());
        for px in pixels {
            let r = px[0] as f32;
            let g = px[1] as f32;
            let b = px[1] as f32;
            let avg: f32 = (r + g + b) / 3.0;
            avg_pixels.push((avg * (2.0/255.0) -1.0));
        }

        println!("{}", avg_pixels.len());
        return Ok(avg_pixels);
    }
}

