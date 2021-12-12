#[cfg(test)]
mod test {
    //use visuaudi::*;
    use image::io::Reader;
    use round::round;
    use std::f32::consts::PI;

    fn mvn_avg_filt(num_points: usize, input_vec: &[f32]) -> Option<Vec<f32>> {
        if num_points < input_vec.len() {
            // To avoid constantly casting
            let points: f32 = num_points as f32;
            let mut output_vec: Vec<f32> = Vec::with_capacity(input_vec.len() + num_points);
            output_vec.push(0.0);

            // Set up 1st element
            for i in 0..num_points {
                // O(1)
                output_vec[0] += input_vec[i];
            }
            output_vec[0] /= points;

            // Applying filter...
            for i in 1..(input_vec.len() - num_points) {
                // O(n)
                output_vec.push(
                    output_vec[i - 1]
                        - (input_vec[i - 1] - input_vec[(i + num_points) - 1]) / points,
                );
            }

            // Smoothing out the last samples with exponential decay...
            // Note that smoothing happens before end of the
            // input vector length.
            for i in (input_vec.len() - num_points)..output_vec.capacity() {
                // O(1)
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
        for x in 0..sinv.capacity() {
            // This is roughly two cycles cuz 12 ~= 2 * (2 * pi)
            sinv.push((x as f32).sin());
        }
        // Sanity check
        assert_eq!(sinv.len(), 12);

        let np1: f32 = 3.0;
        let np2: f32 = 4.0;
        let np3: f32 = 5.0;

        // creating the output vector
        let mut result1: Vec<f32> = Vec::with_capacity(sinv.len() + np1 as usize);
        for r in 0..(sinv.len() - np1 as usize) {
            let x: f32 = (sinv[r] + sinv[r + 1] + sinv[r + 2]) / np1;
            result1.push(x);
        }
        for r in (sinv.len() - np1 as usize)..result1.capacity() {
            let x: f32 = result1[r - 1] / np1;
            result1.push(x);
        }
        assert_eq!(result1.len(), 15);
        let filter_result1: Option<Vec<f32>> = mvn_avg_filt(np1 as usize, sinv.as_slice());
        // There are very small rounding errors
        match filter_result1 {
            Some(fr) => {
                assert_eq!(fr.len(), result1.len());
                for r in 0..fr.len() {
                    assert_eq!(round(fr[r] as f64, 4), round(result1[r] as f64, 4));
                }
            }
            None => {
                assert!(false);
            }
        }

        // creating the output vector
        let mut result2: Vec<f32> = Vec::with_capacity(sinv.len() + np2 as usize);
        for r in 0..(sinv.len() - np2 as usize) {
            let x: f32 = (sinv[r] + sinv[r + 1] + sinv[r + 2] + sinv[r + 3]) / np2;
            result2.push(x);
        }
        for r in (sinv.len() - np2 as usize)..result2.capacity() {
            let x: f32 = result2[r - 1] / np2;
            result2.push(x);
        }
        assert_eq!(result2.len(), 16);
        let filter_result2: Option<Vec<f32>> = mvn_avg_filt(np2 as usize, sinv.as_slice());
        // There are very small rounding errors
        match filter_result2 {
            Some(fr) => {
                assert_eq!(fr.len(), result2.len());
                for r in 0..fr.len() {
                    assert_eq!(round(fr[r] as f64, 4), round(result2[r] as f64, 4));
                }
            }
            None => {
                assert!(false);
            }
        }

        // creating the output vector
        let mut result3: Vec<f32> = Vec::with_capacity(sinv.len() + np3 as usize);
        for r in 0..(sinv.len() - np3 as usize) {
            let x: f32 = (sinv[r] + sinv[r + 1] + sinv[r + 2] + sinv[r + 3] + sinv[r + 4]) / np3;
            result3.push(x);
        }
        for r in (sinv.len() - np3 as usize)..result3.capacity() {
            let x: f32 = result3[r - 1] / np3;
            result3.push(x);
        }
        assert_eq!(result3.len(), 17);
        let filter_result3: Option<Vec<f32>> = mvn_avg_filt(np3 as usize, sinv.as_slice());
        // There are very small rounding errors
        match filter_result3 {
            Some(fr) => {
                assert_eq!(fr.len(), result3.len());
                for r in 0..fr.len() {
                    assert_eq!(round(fr[r] as f64, 4), round(result3[r] as f64, 4));
                }
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_image_reader() {
        let new_img = Reader::open("tests/test_images/287585.jpg");
        match new_img {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
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
        for t in (0..44100).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
        writer.finalize().unwrap();
    }
}
