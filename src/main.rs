mod audio_utils;
mod pixel_utils;
use image::io::Reader as ImageReader;
use image::open;
use std::io::BufReader;
use std::fs::File;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
	// visuaudi --pic=<.JPG> --freq=<FLOAT> --name=<NAME>
	// freq (optional) will change how compressed or
    // stretched the audio frequency is. This should also
    // cause an increase or decrease in pitch, respectively.
    // Example: visuaudi pic.jpg 3.2 new_audio.wav
	// If these 3 are not specified, then fail.

    if arguments.len() >= 3 {
        
        let mut invalid_arg = false; 
        let mut idx = 1; 
        let mut pic: Option<String> = None;
        let mut freq: Option<String> = None;
        let mut name: Option<String> = None;

        // Parsing and initializing provided arguments
        while !invalid_arg && idx < arguments.len() {
            let arg: Vec<&str> = arguments[idx].as_str().split('=').collect();
            match arg[..] {
                ["--pic", p] => { pic = Some(String::from(p)) },
                ["--freq", f] => { freq = Some(String::from(f)) },
                ["--name", n] => { name = Some(String::from(n)) },
                _ => { invalid_arg = true },
            }
            idx += 1;
        }

        if !invalid_arg {
            if let Some(new_pic) = pic {
                // let avg_imgs = pixel_utils::img_to_vec(String::from("ImageFiles/287585.jpg"));
                let avg_imgs = pixel_utils::img_to_vec(new_pic);
                if let Some(new_name) = name {
                    // audio_utils::make_wave(avg_imgs.unwrap(), String::from("GIVE ME A NAME"));
                    audio_utils::make_wave(avg_imgs.unwrap(), new_name);
                } else { // Wave name just takes the name of the file given with .wav extension
                    todo!();
                }
            } else {
                eprintln!("Error: Invalid picture given")
            }
        } else {
            eprintln!("Error: Invlaid argument given")
        }

    }

}
