use std::fs::{self, File};
use std::path::Path;
use std::thread::panicking;
use std::time::SystemTime;

use crate::tgsup;
use crate::pngtool;
use crate::ffmpeg;
pub fn make(workdir : &String , finaldes : String ) {
    let thedir = "".to_owned()+workdir+"/";
    let (TOTFRAME , IFPS ) = tgsup::try_tgs_info(&("".to_owned()+&workdir+"/path.json"));
    let VAL= [512,450,400,375,350,325,300,275,250,225];
    // Change if else below if value is changed
    //Solved
    let FO_FRAME = 18;
    let FO_SCALE = VAL[2];
    let mut OFRAME = FO_FRAME;
    let mut OSCALE =FO_SCALE;
    let mut times : u32 = 2;
    let mut previousf = String::from("");
    loop {
        times+=1;
        // Random code to uncomment to check all quality value the hybrid would make.
        // let (OS, OF) = movethis(OSCALE, OFRAME, &VAL, false, times);
        //             println!("Changing {} , {}FPS for {} to {} , {} FPS ",&OSCALE,&OFRAME,finaldes,OS,OF);
        //             OFRAME=OF;
        //             OSCALE=OS;

        //             if OFRAME == 0 {
        //                 panic!("can't be 0");
        //             }
        //             continue ;
        fs::remove_dir_all("".to_string()+&thedir+"frames");
        fs::remove_dir_all("".to_string()+&thedir+"final");
        fs::create_dir("".to_string()+&thedir+"frames");
        fs::create_dir("".to_string()+&thedir+"final");
        fs::remove_file(&("".to_owned()+&thedir+"output.apng"));


        // New logic to bypass ffmpeg if we have scale it down anyway?? Hopefully fast
        if (FO_SCALE > OSCALE || FO_FRAME > OFRAME) && Path::new(&("".to_string()+&thedir+"output_quant.png")).exists() {
            pngtool::bypass(OSCALE,OFRAME, FO_FRAME, "".to_string()+&thedir+"output_quant.png",&("".to_string()+&thedir+"final/"),("".to_string()+&thedir+"segment.png"));
        }
        else {
            tgsup::tgs_to_png("".to_owned()+&thedir+"path.json" , "".to_owned()+&thedir+"/frames/" , 5, OSCALE, TOTFRAME);
            ffmpeg::launchAt(&("".to_owned()+&thedir+"frames/%d.png"), &("".to_owned()+&thedir+"output.apng"), OSCALE, OFRAME, IFPS);
            pngtool::stich(&("".to_owned()+&thedir+"output.apng"),&("".to_owned()+&thedir+"output_mod.png"), OSCALE);
            let mut this= pngtool::Pngquant::new(&("".to_owned()+&thedir+"output_mod.png"), &("".to_owned()+&thedir+"output_quant.png"));
            this.quantify();
            this.bettercompress(true);
            this.breakinto(&("".to_string()+&thedir+"final/"));

        }
        let delay = (1000.0/OFRAME as f64) as u32;
        pngtool::compressat(&("".to_string()+&thedir+"final/"));
        let randname= SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string();
        let randname = "".to_string()+&thedir+&randname+".apng";
        let fsize= pngtool::makedirect(&("".to_string()+&thedir+"final/"), &randname, delay) ;
        println!("Trying {}WxH at FPS {} on {}.gz got File Size of {}kb", OSCALE, OFRAME, finaldes , fsize);

        // if fsize < 300 {
        //     println!("File made ");
        //     fs::rename(&("".to_string()+&thedir+"made.apng"), "outputdir/");
        // }
        let previ = previousf.clone();
        let finald = "outputdir/".to_string()+&finaldes+".png";

       // println!("Previous {} , Final {} , and This {} ", previ,finald,randname);
        if previousf.eq(""){
            if fsize < 299.9 {
                  if OFRAME==FO_FRAME && OSCALE ==FO_SCALE && fsize < 280.0 {
                      previousf=randname;
                    let (OS, OF) = movethis(OSCALE, OFRAME, &VAL, true, times);
                    // println!("Changing {} , {}FPS for {} to {} , {} FPS ",&OSCALE,&OFRAME,finaldes,OS,OF);
                    OFRAME=OF;
                    OSCALE=OS;
                  }
                  else {
                      fs::rename(&randname, finald).unwrap();
                      break;
                  }
            }
            else{
                let (OS, OF) = movethis(OSCALE, OFRAME, &VAL, false,times);
                // println!("Changing {} , {}FPS for {} to {} , {} FPS ",&OSCALE,&OFRAME,finaldes,OS,OF);
                    OFRAME=OF;
                    OSCALE=OS;
            }

        }
        else  {
            if (OFRAME == 20 && OSCALE == 512) && fsize < 299.9 {
                fs::rename(randname, finald).unwrap();
                break;
            }
            else if fsize < 280.0 {
                previousf=randname;
                let (OS, OF) = movethis(OSCALE, OFRAME, &VAL, true, times);
                // println!("Changing {} , {}FPS for {} to {} , {} FPS ",&OSCALE,&OFRAME,finaldes,OS,OF);
                OFRAME=OF;
                OSCALE=OS;
            }
            else if fsize < 299.9 {
                fs::rename(randname, finald).unwrap();
                break;
            }
            else {
                fs::rename(previ, finald).unwrap();
                break;
            }
        }
    }
    println!("Completed {} " , &finaldes );
}

pub fn movethis(OSCALE : u32 , OFRAME :u32 , VAL : &[u32 ; 10] , goup : bool , times : u32)-> (u32,u32){
    if !goup {
        if OSCALE == 225 {
            return (OSCALE,OFRAME-1)
        }
        else {
            let scaleat = VAL.iter().position(|&r| r==OSCALE).unwrap();
            if times%3 != 0 {
                return (VAL[scaleat+1],OFRAME);
            }
            else {
                return (OSCALE,OFRAME-1);
            }
        }
    }
    else {
        if OSCALE == 512 {
            return (OSCALE,20);
        }
        else {
            // println!("Moving up {}", OSCALE);
            let scaleat = VAL.iter().position(|&r| r==OSCALE).unwrap(); 
            if times%3 == 0 {
                return (VAL[scaleat-1],OFRAME);
            }
            else {
                return (OSCALE,OFRAME+1);
            }
        }
    }
}

//102 and 74 are bugged check again later,
// Checked
