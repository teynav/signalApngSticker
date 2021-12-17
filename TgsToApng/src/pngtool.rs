extern crate png;
extern crate image;
extern crate imagequant;
extern crate oxipng;
extern crate threadpool;
extern crate apng;
use std::{convert::TryInto, fs::File};

use image::AnimationDecoder;
//use apng::DisposeOp;
//use image::ColorType;
use image::codecs::png::{PngEncoder, ApngDecoder , PngDecoder};
use png::{BlendOp, Decoder, Encoder, Info};
use oxipng::{InFile,OutFile, Options};
use image::io::Reader as ImgReader;
use std::io::BufWriter;
use std::path::PathBuf;
use threadpool::ThreadPool;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
pub struct Pngquant {
    name : String,
    height : u32 ,
    width : u32,
    newfile : String,
}

 impl Pngquant {




    pub fn new( s : &str , s1 : &str) -> Pngquant {
        return Pngquant {
            name : s.to_owned(),
            height : 0 ,
            width : 0 ,
            newfile : s1.to_owned(),
        }
    }




    pub fn quantify (&mut self) -> (Vec<u8> ,Vec<u8>){
        let decoding = ImgReader::open(&self.name).unwrap().decode().unwrap().into_rgba8();
        self.width = decoding.width();
        self.height = decoding.height();
        let imgpix = decoding.pixels();

        
        let mut imgq = imagequant::new();
        imgq.set_speed(3);
        imgq.set_quality(60, 70);
        imgq.set_last_index_transparent(true);
        let mut encc = vec![];
        for pixel in imgpix {
             let collect = pixel.0;
             encc.push(imagequant::RGBA{r:collect[0], g:collect[1] , b:collect[2] , a:collect[3]  });

        }
        let ref mut thisnewenc = imgq.new_image(&encc, self.width.try_into().unwrap()  , self.height.try_into().unwrap()  , 0.0).unwrap();
        let  mut colorp = vec![] ; 
        let  mut pixels = vec![] ; 
        let  temp = imgq.quantize(&thisnewenc);
        let mut temp1;

        match temp {
            Ok(a) => temp1=a, 
            Err(_)=> {
                   println!("Issue with {} Increasing Quality ", &self.name);
                  let mut imgq1 = imagequant::new();
                  imgq1.set_speed(3);
                  imgq1.set_quality(60, 90);
                  imgq1.set_last_index_transparent(true);
                  let ref mut thisnewenc1 = imgq1.new_image(&encc, self.width.try_into().unwrap()  , self.height.try_into().unwrap()  , 0.0).unwrap();
                  temp1 = imgq1.quantize(&thisnewenc1).unwrap();
                 }
        }
        match temp1.remapped(thisnewenc) {
            Ok((x,y)) => {
                colorp=x;
                pixels=y;
                },
            Err(_) => () 
        }

        let  bufw = BufWriter::new(File::create(&self.newfile).unwrap());
        let mut newpng = Encoder::new(bufw,self.width,self.height);
        newpng.set_depth(png::BitDepth::Eight);
        newpng.set_compression(png::Compression::Best);
        newpng.set_color(png::ColorType::Indexed);
        let mut tran = vec![];
        let mut thispal  = vec![];
        for entry in colorp {
              thispal.push(entry.r); thispal.push(entry.g);   thispal.push(entry.b);  tran.push(entry.a);    
        }
        newpng.set_trns(&tran);
        newpng.set_palette(&thispal);
        let mut writeme = newpng.write_header().unwrap();
        writeme.write_image_data( &pixels ).unwrap();
        (thispal,tran)
    }





    pub fn bettercompress (&self , sameoutput : bool) {
        let mut opt = Options::max_compression();
        //opt.use_heuristics=false;
        //opt.strip=oxipng::Headers::Safe;
        //opt.interlace=Some(1);
        if !sameoutput {
            oxipng::optimize(&InFile::Path(PathBuf::from(&self.name)), &OutFile::Path(Some(PathBuf::from(&self.newfile))), &opt).unwrap();
        }
        else {
            oxipng::optimize(&InFile::Path(PathBuf::from(&self.newfile)), &OutFile::Path(None), &opt).unwrap();
        }
    }




    pub fn breakinto (&self , locate : &str  ) {
        let totaltimes = self.height/self.width;
        if totaltimes == 1{
            return ;
        } 
        let mut counter =0 ;
        let mut decoding = ImgReader::open(&self.newfile).unwrap().decode().unwrap();
        while counter < totaltimes {
            let crop = decoding.crop(0 , counter*self.width, self.width, self.width);
            crop.save("".to_owned()+locate+""+&counter.to_string()+".png").unwrap();
            counter=counter+1;
        }
    }







// 
//     pub fn makeani (s : &str , output : &str , thispal : &[u8] , tran : &[u8] ) {
//         let mut counter=0;
//         let mut width =0  ;
//         let mut height =0;
//         let mut collection : Vec<Vec<u8>> = vec![];
//         loop {
//             let filename ="".to_owned()+s+&counter.to_string()+".png";
//             let result = File::open(&filename);
//             match result {
//                 Ok(ref file) => {
//                     let decoder = Decoder::new(file);
//                     let mut reader = decoder.read_info().unwrap();
//                     let mut buf = vec![0;reader.output_buffer_size()];
//                     let info = reader.next_frame(&mut buf).unwrap();
//                     collection.push(buf);
//                     width = info.width;
//                     height = info.height;
//                 } ,
//                 Err(_) => break
//             }
//             counter=counter+1;
//         }
// 
//         let mut bufw = BufWriter::new(File::create(output).unwrap());
//         let mut  encoder = Encoder::new(bufw, width , height);
//         encoder.set_animated(counter, 0);
//         encoder.set_filter(png::FilterType::Sub);
//         encoder.set_compression(png::Compression::Best);
//         encoder.set_depth(png::BitDepth::Eight);
//         encoder.set_color(png::ColorType::Rgb);
//         encoder.set_frame_delay(1, 10).unwrap();
//         encoder.set_palette(thispal);
//         encoder.set_trns(tran);
//         encoder.set_blend_op(BlendOp::Over);
//         encoder.set_dispose_op(png::DisposeOp::Previous);
//         encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);
// 
//         let mut writerofall = encoder.write_header().unwrap();
// 
//         for eachframe in collection {
//             println!("length {} capacity {}  " , eachframe.len() ,eachframe.capacity());
//             writerofall.write_image_data(eachframe.as_slice()).unwrap();
//         }
//    }
      
//      pub fn makeani (location : &str ){
//          let mut counter=0;
//          let mut collection = vec![];
//          loop {
//             let filename ="".to_owned()+location+&counter.to_string()+".png";
//             let result = File::open(&filename);
//             match result {
//                 Ok(ref file) => {
//                     collection.push(apng::load_png(&filename).unwrap());
//                 }
//                 Err(_) => break 
//             }
//             counter=counter+1;
//          }
//         let path = Path::new(r"out.png");
//         let mut out = BufWriter::new(File::create(path).unwrap());
//         let config = apng::create_config(&collection, None).unwrap();
//         let mut encoder = apng::Encoder::new(&mut out, config).unwrap();
//         let frame = apng::Frame {
//             delay_num: Some(1),
//             delay_den: Some(10),
//             dispose_op: Some(apng::DisposeOp::ApngDisposeOpBackground),
//             blend_op : Some(apng::BlendOp::ApngBlendOpOver),
//             ..Default::default()
//         };
//         match encoder.encode_all(collection, Some(&frame)) {
//             Ok(_n) => println!("success"),
//             Err(err) => eprintln!("{}", err),
//         }
// 
//      }
}


    pub fn compressat (s : &str) {
        let mut counter = 0; 
        let jobs = ThreadPool::new(5);
        // println!("Optimizing ");
        loop {
            let filename ="".to_owned()+s+&counter.to_string()+".png";
            let result = File::open(&filename);
            let file;
            match result {
                Ok(ref now) => file=now ,
                Err(_) => break
            }
            counter+=1;
            let mut opt = Options::max_compression();
            let fname = filename.clone();
            jobs.execute( move || {
                           oxipng::optimize(&InFile::Path(PathBuf::from(&fname)), &OutFile::Path(None), &opt).unwrap();
            });
        }
        // println!("Optimized {}",counter);
        jobs.join();
    }





pub fn makedirect (s : &str , out : &str , delay : u32 ) -> f32 {
     let mut counter=0;
     let mut results = vec![];
    loop {
        let filename ="".to_owned()+s+&counter.to_string()+".png";
        let result = File::open(&filename);
        match result {
            Ok(_) => {
                 results.push(filename);
            }
                Err(_) => break 
            }
            counter=counter+1;
         }
    let child = Command::new("apngasm").arg("-F").arg("-d")
                                                 .arg(delay.to_string())
                                                 .arg("-o").arg(out).args(results)
                                                 .stderr(std::process::Stdio::piped())
                                                 .stdout(std::process::Stdio::piped())
                                                 .spawn().expect("Failed to start the process");
    let output = child.wait_with_output().expect("Error Collecting output");
    // println!("{:?}",String::from_utf8(output.stdout.as_slice().to_vec()));
    // println!("{:?}",&results); 
    let finalstatus = std::fs::metadata(out).unwrap();
    let filesize  = finalstatus.len() as f32;
    filesize/1024.0
}

// pub fn stich(name : &str, output : &str , dataw : u32) {
//       let thisapng = PngDecoder::new(File::open(name).unwrap()).unwrap();
//       let theapng = thisapng.apng();
//       let frames = theapng.into_frames();
//       
//       let bufw = BufWriter::new(File::create(output).unwrap());
//       let newpng = PngEncoder::new_with_quality(bufw, image::png::CompressionType::Fast, image::png::FilterType::Sub);
//       let mut imagecont = vec![];
//       let mut found = 0 ;
//       for frame in frames {
//           found +=1;
//           let buf=frame.unwrap().into_buffer();
//           let  buf= buf.pixels();
//           for pix in buf {
//                   imagecont.push(pix.0[0]);
//                   imagecont.push(pix.0[1]);
//                   imagecont.push(pix.0[2]);
//                   imagecont.push(pix.0[3]);
//           }
//       }
//       
//       newpng.encode(&imagecont, dataw, dataw*found, image::ColorType::Rgba8).unwrap();
// }
// 


 pub fn stich(name : &str, output : &str , dataw : u32) {

    let mut child = Command::new("apngdis").arg(name).arg("-s").stdout(Stdio::null()).spawn().expect("Failed to run the apngdis, exiting");
    if ! child.wait().expect("Failed to run apngdis").success() {
        panic!("Can't run apngdis");
    }
    child.wait();
}
pub fn bypass(SCALE : u32,FRAME : u32,PFRAME : u32, input : String , locate : &str, tempo : String){
    let x= ImgReader::open(&input).unwrap();
    let (w,h) = x.into_dimensions().unwrap();
    let totframe = h/w;
    let tempo1 = "".to_owned()+&tempo+"_quant.png";
    let x= ImgReader::open(&input).unwrap().decode().unwrap().thumbnail(SCALE, SCALE*totframe).save(&tempo);
    let mut pp =Pngquant::new(&tempo, &tempo1);
    pp.quantify();
    pp.bettercompress(true);

    let finalf = ((FRAME*totframe) as f64 / PFRAME as f64) as u32;
    let tot_d_by = totframe - finalf;
    // println!("To Do removal of {} changing scale  {} to {} and fps to {} from the given {}", tot_d_by, w, SCALE ,FRAME , PFRAME);
    let mut counter =0 ;
    let mut count = 0;
    let mut decoding = ImgReader::open(tempo1).unwrap().decode().unwrap();
    while counter < totframe {
        if tot_d_by != 0 && counter !=0 && counter %(totframe/tot_d_by) == 0 {
            if tot_d_by !=1 {
                counter=counter+1;
                continue;
            }
            else if totframe/2 == counter {
                counter=counter+1;
                continue;
            }
            else {
                counter=counter+1;
            }
        }
        let  crop = decoding.crop(0 , SCALE*counter, SCALE, SCALE);
        crop.thumbnail_exact(SCALE,SCALE).save_with_format("".to_owned()+locate+""+&count.to_string()+".png", image::ImageFormat::Png).unwrap();
        counter=counter+1;
        count+=1;
    }

    // panic!("Check")
}

