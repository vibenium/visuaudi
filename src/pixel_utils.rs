use image::open;

fn mvn_avg_filt(num_points: usize, input_vec: &[f32]) -> Option<Vec<f32>> {
    if num_points < input_vec.len() {

        // To avoid constantly casting
        let points: f32 = num_points as f32;
        let mut output_vec: Vec<f32>
            = Vec::with_capacity(input_vec.len() + num_points);
        output_vec.push(0.0);

        // Set up 1st element
        for i in 0..num_points { // O(1)
            output_vec[0] += input_vec[i];
        }
        output_vec[0] /= points;

        // Applying filter...
        for i in 1..(input_vec.len() - num_points) { // O(n)
            output_vec.push(
                output_vec[i - 1] -
                (input_vec[i - 1] - input_vec[(i + num_points) - 1]) /
                points);
        }

        // Smoothing out the last samples with exponential decay...
        // Note that smoothing happens before end of the
        // input vector length.
        for i in (input_vec.len() - num_points)..output_vec.capacity() { // O(1)
            output_vec.push(output_vec[i - 1] / points);
        }
        return Some(output_vec);
    }
    return None;
}

pub fn img_to_vec(path: String, num_pts: Option<String>) -> Result<Vec<f32>, &'static str> {

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
            let b = px[2] as f32;
            let avg: f32 = (r + g + b) / 3.0;
            avg_pixels.push((avg * (2.0/255.0) -1.0));
        }
        
        if let Some(s) = num_pts { // Apply filter
            if let Ok(num) = s.parse::<usize>() { // Convert String to usize
                match mvn_avg_filt(num, avg_pixels.as_slice()) {
                    Some(filtered_data) => Ok(filtered_data),
                    None => Err("Error: Moving average filter failed"),
                }
            } else {
                Err("Error: Failed to parse num_points parameter as usize")
            }
        } else { // Return unfiltered data
            Ok(avg_pixels) 
        }
        /*
        let filtered_data = mvn_avg_filt(4, avg_pixels.as_slice());
        match filtered_data {
            Some(fd) => Ok(fd),
            None => Err("Image filtered failed"),
        }*/
        // println!("{}", avg_pixels.len());
        //return Ok(avg_pixels);
    }
}

