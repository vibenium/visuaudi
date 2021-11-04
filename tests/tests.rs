
#[cfg(test)]
mod test {
    //use visuaudi::*;
	use std::f32::consts::PI;
    use image::io::Reader;
    use std::path::Path;
    // use image::ImageFormat;
    #[test]
    fn it_works() {
        assert!(true);
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
