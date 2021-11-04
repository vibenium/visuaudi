mod audio_utils;
mod pixel_utils;
use audio_utils::*;
use pixel_utils::*;
use image::io::Reader;
use std::io::BufReader;
use std::fs::File;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
	// visuaudi pic=<.JPG> time=<FLOAT> name=<NAME>
	// Example: visuaudi pic.jpg 3.2 new_audio.wav
	// If these 3 are not specified, then fail.
    if arguments.len() == 4 {
        let new_img: Option<Reader<BufReader<File>>>
            = match Reader::open(arguments[1].as_str()) {
            Ok(r) => Some(r),
            Err(_) => None
        };
        let time: Option<u32> = match arguments[2].as_str().parse::<u32>() {
            Ok(t) => Some(t),
            Err(_) => None,
        };
        // name does not need to be analyzed (probobaly)
        // ...
        todo!();
    } else {
        eprintln!("visuaudi takes 3 arguments...");
        eprintln!("\tpic=<.JPG> time=<FLOAT> name=<NAME>");
    }
}
