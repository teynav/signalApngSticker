use std::fs;
use std::path::Path;
use std::time::SystemTime;

use crate::ffmpeg;
use crate::pngtool;
use crate::tgsup;
pub fn make(workdir: &String, finaldes: String, is_video: bool, initial: String) {
    let thedir = "".to_owned() + workdir + "/";
    let tot_frame: u32;
    let input_fps: u32;
    if !is_video {
        (tot_frame, input_fps) = tgsup::try_tgs_info(&("".to_owned() + &workdir + "/path.json"));
    } else {
        (tot_frame, input_fps) = ffmpeg::get_info_webm(&initial);
        println!("tot_frame {}", &tot_frame);
        println!("input_fps {}", &input_fps);
    }
    let val = [512, 450, 400, 375, 350, 325, 300, 275, 250, 225];
    // Change if else below if value is changed
    //Solved
    let fo_frame = 18;
    let fo_scale = val[2];
    let mut oframe = fo_frame;
    let mut oscale = fo_scale;
    let mut times: u32 = 2;
    let mut previousf = String::from("");
    loop {
        times += 1;
        // Random code to uncomment to check all quality value the hybrid would make.
        // let (OS, OF) = movethis(oscale, oframe, &val, false, times);
        //             println!("Changing {} , {}FPS for {} to {} , {} FPS ",&oscale,&oframe,finaldes,OS,OF);
        //             oframe=OF;
        //             oscale=OS;

        //             if oframe == 0 {
        //                 panic!("can't be 0");
        //             }
        //             continue ;
        fs::remove_dir_all("".to_string() + &thedir + "frames").ok();
        fs::remove_dir_all("".to_string() + &thedir + "final").ok();
        fs::create_dir("".to_string() + &thedir + "frames").ok();
        fs::create_dir("".to_string() + &thedir + "final").ok();
        fs::remove_file(&("".to_owned() + &thedir + "output.apng")).ok();
        // New logic to bypass ffmpeg if we have scale it down anyway?? Hopefully fast
        if (fo_scale > oscale || fo_frame > oframe)
            && Path::new(&("".to_string() + &thedir + "output_quant.png")).exists()
        {
            pngtool::bypass(
                oscale,
                oframe,
                fo_frame,
                "".to_string() + &thedir + "output_quant.png",
                &("".to_string() + &thedir + "final/"),
                "".to_string() + &thedir + "segment.png",
            );
        } else {
            if is_video {
                ffmpeg::extract_vp9(
                    "".to_owned() + &thedir + "output.apng",
                    &initial,
                    oscale,
                    oframe,
                    input_fps,
                );
            } else {
                tgsup::tgs_to_png(
                    "".to_owned() + &thedir + "path.json",
                    "".to_owned() + &thedir + "/frames/",
                    5,
                    oscale,
                    tot_frame,
                );
                ffmpeg::launch_at(
                    &("".to_owned() + &thedir + "frames/%d.png"),
                    &("".to_owned() + &thedir + "output.apng"),
                    oscale,
                    oframe,
                    input_fps,
                );
            }

            pngtool::stich(
                &("".to_owned() + &thedir + "output.apng"),
                &("".to_owned() + &thedir + "output_strip.png"),
                oscale,
            );
            let mut this = pngtool::Pngquant::new(
                &("".to_owned() + &thedir + "output_strip.png"),
                &("".to_owned() + &thedir + "output_quant.png"),
            );
            this.quantify();
            this.bettercompress(true);
            this.breakinto(&("".to_string() + &thedir + "final/"));
        }
        let delay = (1000.0 / oframe as f64) as u32;
        pngtool::compressat(&("".to_string() + &thedir + "final/"));
        let randname = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let randname = "".to_string() + &thedir + &randname + ".apng";
        let fsize = pngtool::makedirect(&("".to_string() + &thedir + "final/"), &randname, delay);
        println!(
            "Trying {}WxH at FPS {} on {}.gz got File Size of {}kb",
            oscale, oframe, finaldes, fsize
        );

        // if fsize < 300 {
        //     println!("File made ");
        //     fs::rename(&("".to_string()+&thedir+"made.apng"), "outputdir/");
        // }
        let previ = previousf.clone();
        let finald = "outputdir/".to_string() + &finaldes + ".png";

        // println!("Previous {} , Final {} , and This {} ", previ,finald,randname);
        if previousf.eq("") {
            if fsize < 299.9 {
                if oframe == fo_frame && oscale == fo_scale && fsize < 280.0 {
                    previousf = randname;
                    let (os, of) = movethis(oscale, oframe, &val, true, times);
                    // println!("Changing {} , {}FPS for {} to {} , {} FPS ",&oscale,&oframe,finaldes,os,of);
                    oframe = of;
                    oscale = os;
                } else {
                    fs::rename(&randname, finald).unwrap();
                    break;
                }
            } else {
                let (os, of) = movethis(oscale, oframe, &val, false, times);
                // println!("Changing {} , {}FPS for {} to {} , {} FPS ",&oscale,&oframe,finaldes,os,of);
                oframe = of;
                oscale = os;
            }
        } else {
            if (oframe == 20 && oscale == 512) && fsize < 299.9 {
                fs::rename(randname, finald).unwrap();
                break;
            } else if fsize < 280.0 {
                previousf = randname;
                let (os, of) = movethis(oscale, oframe, &val, true, times);
                // println!("Changing {} , {}FPS for {} to {} , {} FPS ",&oscale,&oframe,finaldes,os,of);
                oframe = of;
                oscale = os;
            } else if fsize < 299.9 {
                fs::rename(randname, finald).unwrap();
                break;
            } else {
                fs::rename(previ, finald).unwrap();
                break;
            }
        }
    }
    println!("Completed {} ", &finaldes);
}

pub fn movethis(oscale: u32, oframe: u32, val: &[u32; 10], goup: bool, times: u32) -> (u32, u32) {
    if !goup {
        if oscale == 225 {
            return (oscale, oframe - 1);
        } else {
            let scaleat = val.iter().position(|&r| r == oscale).unwrap();
            if times % 3 != 0 {
                return (val[scaleat + 1], oframe);
            } else {
                return (oscale, oframe - 1);
            }
        }
    } else {
        if oscale == 512 {
            return (oscale, 20);
        } else {
            // println!("Moving up {}", oscale);
            let scaleat = val.iter().position(|&r| r == oscale).unwrap();
            if times % 3 == 0 {
                return (val[scaleat - 1], oframe);
            } else {
                return (oscale, oframe + 1);
            }
        }
    }
}
