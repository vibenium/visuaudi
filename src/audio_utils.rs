

use hound;
// Put averaged rgb values into an array such that
// each value is subtracted by 127

// Equation: (val * (2/255)) - 1


pub fn make_wave(samples: Vec<f32>, name: String) {
	let spec = hound::WavSpec {
		channels: 1,
		sample_rate: 44100,
		bits_per_sample: 32, 
		sample_format: hound::SampleFormat::Int,
	};
	let mut writer = hound::WavWriter::create(name, spec).unwrap();
	for s in samples {
		let amplitude = i32::MAX as f32;
		writer.write_sample((s * amplitude) as i32).unwrap();
	}
	writer.finalize().unwrap();
}
