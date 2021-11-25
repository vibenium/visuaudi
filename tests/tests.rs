#[cfg(test)]
mod test {
    //use visuaudi::*;
	use std::f32::consts::PI;
    use image::io::Reader;
    use std::path::Path;
    // use image::ImageFormat;
    use image::ImageBuffer;

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
                    (input_vec[i - 1] + input_vec[(i + num_points) - 1]) / 
                    points);
            }
           
            // Smoothing out the last samples with exponential decay...
            for i in input_vec.len()..output_vec.len() { // O(1)
                output_vec.push(output_vec[i - 1] / points);
            }
            return Some(output_vec); 
        }
        return None;
    }

    #[test]
    fn test_moving_avg_filter_algo() {
        
        // Constructing sine wave vector
        let mut sinv: Vec<f32> = Vec::with_capacity(12);
        for x in 0..12 { // This is roughly two cycles cuz 12 ~= 2 * (2 * pi)
            sinv.push((x as f32).sin());
        }
        
        let np1: usize = 3;
        let np2: usize = 4;
        let np3: usize = 5;
/*
        v1: Vec<f32> = mvn_avg_filt(np1, sinv.as_slice(), v1.as_mut_slice());
       
        // TESTING filter result...
        for x in 0..(sinv.len() - 2) {
            let this_avg = (sinv[x] + sinv[x + 1] + sinv[x + 2]) / 3.0;
            println!("DEBUG: x @ {}, this_avg = {}", x, this_avg);
            println!("WHEN v1[{}] = {}", x, v1[x]);
            assert_eq!(v1[x], this_avg); 
        } */
    }
        
    #[test]
    fn test_image_reader() {
        let new_img = Reader::open("tests/test_images/287585.jpg");
        match new_img {
            Ok(_) => assert!(true),
            Err(e) => assert!(false),
        }

    }

	#[test] // Making sure .WAV writer works
    fn gen_sine_wave() {
		// The code below was taken from the hound crate example
        let spec = hound::WavSpec {
			channels: 1,
			sample_rate: 44100,
			bits_per_sample: 16,
			sample_format: hound::SampleFormat::Int,
		};
		let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
		for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
			let sample = (t * 440.0 * 2.0 * PI).sin();
			let amplitude = i16::MAX as f32;
			writer.write_sample((sample * amplitude) as i16).unwrap();
		}
		writer.finalize().unwrap();
    }
}
