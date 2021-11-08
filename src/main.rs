mod audio_utils;
mod pixel_utils;
use image::io::Reader as ImageReader;
use image::open;
use std::io::BufReader;
use std::fs::File;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
	// visuaudi pic=<.JPG> time=<FLOAT> name=<NAME>
	// Example: visuaudi pic.jpg 3.2 new_audio.wav
	// If these 3 are not specified, then fail.

    let vec = pixel_utils::img_to_vec(String::from("ImageFiles/287585.jpg"));
    println!("{}", vec.unwrap().len());
}
