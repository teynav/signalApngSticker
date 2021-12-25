use std::process::Command;
use std::process::Stdio;

pub fn launch_at(location: &str, output: &str, resout: u32, fpsout: u32, fpsin: u32) {
    let settings = "scale=".to_owned() + &resout.to_string() + ":-1:flags=neighbor:sws_dither=none";
    let _child = Command::new("ffmpeg")
        .arg("-r")
        .arg(&fpsin.to_string())
        .arg("-i")
        .arg(&location)
        .arg("-vcodec")
        .arg("apng")
        .arg("-vf")
        .arg(&settings)
        .arg("-r")
        .arg(&fpsout.to_string())
        .arg("-plays")
        .arg("0")
        .arg(&output)
        .stdout(Stdio::null())
        .output()
        .expect("Failed to Launch FFMPEG");
}
