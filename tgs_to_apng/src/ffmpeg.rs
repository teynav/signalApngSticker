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

pub fn get_info_webm(file: &String) -> (u32,u32) {
    let settings ="default=nokey=1:noprint_wrappers=1";
    let output = Command::new("ffprobe").arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-count_frames")
        .arg("-show_entries")
        .arg("stream=nb_read_frames")
        .arg("-print_format")
        .arg(&settings)
        .arg(&file)
        .output()
        .expect("Failed with ffprobe");
    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();
    let frames :u32 = stdout.trim().parse().unwrap();

    let settings="stream=r_frame_rate";
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("0")
        .arg("-of")
        .arg("csv=p=0")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg(&settings)
        .arg(&file)
        .output()
        .expect("Failed with ffprobe");

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    let mut second =String::from("");
    let mut first=String::from("");
     let find = stdout.rfind("/");
            match find {
                Some(xx) => {
                    first = stdout[..xx].to_owned();
                    second= stdout[xx+1..].to_owned();
                }
                None => (),
            }
    first=first.trim().to_owned();
    second=second.trim().to_owned();
    let int1 : u32 = first.parse().unwrap();
    let int2 : u32 = second.parse().unwrap();
    (frames,int1/int2)
}

pub fn extract_vp9(location : String , name : &String , scale : u32 , frame : u32 , _i_fps : u32 ) {
    
// ffmpeg -c:v libvpx-vp9 -y -i 4.webm -vcodec apng -f apng -pix_fmt rgba -vf "scale=512:-1:flags=neighbor:sws_dither=none,pad=512:512:(ow-iw)/2:(oh-ih)/2:color=black@0,setsar=1" -r 22 -plays 0 frames/output.apng 
 //  let settings ="scale=512:-1:flags=neighbor:sws_dither=none,scale=512:512:force_original_aspect_ratio=decrease,pad=512:512:(ow-iw)/2:(oh-ih)/2:color=black@0,setsar=1" ;
    let scale_s = scale.to_string();
    let frame_s= frame.to_string();
    let settings=  "scale=".to_owned()+&scale_s+":-1:flags=neighbor:sws_dither=none,pad="+&scale_s+":"+&scale_s+":(ow-iw)/2:(oh-ih)/2:color=black@0,setsar=1";
    let child = Command::new("ffmpeg").arg("-c:v")
       .arg("libvpx-vp9")
       .arg("-y")
       .arg("-i")
       .arg(&name)
       .arg("-vcodec")
       .arg("apng")
       .arg("-f")
       .arg("apng")
       .arg("-pix_fmt")
       .arg("rgba")
       .arg("-vf")
       .arg(&settings)
       .arg("-r")
       .arg(&frame_s)
       .arg("-plays")
       .arg("0")
       .arg(&location)
       .stdout(Stdio::null())
       .output()
       .expect("ffmpeg failed !");
    let _ = String::from_utf8(child.stderr).unwrap();
}
