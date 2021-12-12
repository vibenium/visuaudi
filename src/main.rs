mod audio_utils;
mod pixel_utils;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    // visuaudi --pic=<.JPG> --freq=<FLOAT> --name=<NAME> --pts=<num_points>
    // freq (optional) will change how compressed or
    // stretched the audio frequency is. This should also
    // cause an increase or decrease in pitch, respectively.
    // Example: visuaudi pic.jpg 3.2 new_audio.wav
    // If these 3 are not specified, then fail. pts (optional)
    // represents the number of points that will be used in
    // the moving average filter. It is recommended that this
    // option be filled to reduce excessive noise. However, this
    // can be omitted.

    if arguments.len() >= 3 {
        let mut invalid_arg = false;
        let mut idx = 1;
        let mut pic: Option<String> = None;
        let mut freq: Option<String> = None;
        let mut name: Option<String> = None;
        let mut num_points: Option<String> = None;

        // Parsing and initializing provided arguments
        while !invalid_arg && idx < arguments.len() {
            let arg: Vec<&str> = arguments[idx].as_str().split('=').collect();
            match arg[..] {
                ["--pic", p] if pic.is_none() => pic = Some(String::from(p)),
                ["--freq", f] if freq.is_none() => freq = Some(String::from(f)),
                ["--name", n] if name.is_none() => name = Some(String::from(n)),
                ["--pts", p] if num_points.is_none() => num_points = Some(String::from(p)),
                _ => invalid_arg = true,
            }
            idx += 1;
        }

        if !invalid_arg {
            if let Some(new_pic) = pic {
                let avg_imgs = pixel_utils::img_to_vec(new_pic, num_points);
                if let Some(new_name) = name {
                    audio_utils::make_wave(avg_imgs.unwrap(), new_name);
                } else {
                    // Wave name just takes the name of the file given with .wav extension
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
