extern crate flate2;
extern crate threadpool;

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::Command;
use std::process::Stdio;

pub fn try_upload(
    program: &str,
    pack_name: &str,
    pack_author: &str,
    userid: &str,
    pass: &str,
    emoji: &str,
    location: &str,
    start: u32,
    end: u32,
) -> bool {
    let mut child = Command::new(&program)
        .arg("uploader.py")
        .arg(&pack_name)
        .arg(&pack_author)
        .arg(&userid)
        .arg(&pass)
        .arg(&emoji)
        .arg(&location)
        .arg(&start.to_string())
        .arg(&end.to_string())
        .spawn()
        .expect("Failed to run the task, exiting");
    if !child.wait().expect("Failed to run").success() {
        return false;
    }
    true
}
pub fn try_decode(file: &str, location: &str) {
    let mut bufr = BufReader::new(File::open(file).unwrap());
    let mut content = vec![];
    bufr.read_to_end(&mut content).ok();
    let mut decoded = GzDecoder::new(&content[..]);
    let mut decres = "".to_owned();
    decoded.read_to_string(&mut decres).ok();
    let mut bufw = BufWriter::new(File::create(location).unwrap());
    bufw.write_all(&decres.as_bytes()).unwrap();
}

pub fn try_tgs_info(file: &str) -> (u32, u32) {
    let child = Command::new("tgs2png")
        .arg("-i")
        .arg(&file)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output1 = child.wait_with_output().expect("Can't read file");

    let output = String::from_utf8(output1.stdout.to_vec()).unwrap();
    let mut frames = output.find("frames=").unwrap();
    let output = &output[frames + 7..];
    frames = output.find(",").unwrap();

    let finalframe: u32 = output[0..frames].parse().unwrap();

    frames = output.find("fps=").unwrap();
    let output = &output[frames + 4..];
    frames = output.find(".").unwrap();

    let finalfps: u32 = output[0..frames].parse().unwrap();

    (finalframe, finalfps)
}

pub fn tgs_to_png(file: String, location: String, threads: usize, theres: u32, frames: u32) {
    let res = theres * 4;
    let mut counter = 0;
    let pool = threadpool::ThreadPool::new(threads);
    while counter < frames {
        let file = file.clone();
        let location = location.clone();
        pool.execute(move || {
            let (shell, flag) = if cfg!(windows) {
                ("cmd.exe", "/C")
            } else {
                ("sh", "-c")
            };
            let shell_command = "tgs2png -s ".to_owned()
                + &res.to_string()
                + "x0 -o "
                + &counter.to_string()
                + " -n 1  "
                + &file
                + " > "
                + &location
                + &counter.to_string()
                + ".png";
            let mut child = Command::new(shell);
            child.arg(flag);
            child.arg(shell_command);
            child.spawn().expect("Failed to run").wait().ok();
        });
        counter = counter + 1;
    }
    pool.join();
}
