use std::fs::{self, File , OpenOptions};
use std::path::Path;
use std::io::{self, BufReader, prelude::*};
use std::process::{Command, Stdio};
use std::env;
use threadpool::ThreadPool;

mod tgsGet;
mod tgsup;
mod makeapng;
mod pngtool;
mod ffmpeg;
use std::time::Instant;
// use image::io::Reader;
// fn deletebak() {
//     std::fs::remove_dir_all(".backup");
//     std::fs::remove_file(".back");
// }


fn firstinstall() {
    if !Path::new("token").exists() && !Path::new("token").is_file() {
        let file = OpenOptions::new().read(true).write(true).create(true).append(false).open("token") .unwrap();
        println!("\nTelegram Bot Token not found, \nPlease use v1 if you have tgs , \nv3 always downloads tgs requires bot token\nYou can input token now or exit");
        let  teletok=input();
        println!("\nEnter default author's name");
        let  author=input();
        println!("\n\nNow open Signal Desktop ,\nGoto Menu -> Toggle Developers tools -> On there open Console \nPaste output of window.reduxStore.getState().items.uuid_id");
        let  user=input();
        println!("You are almost there \nPaste output of window.reduxStore.getState().items.password");
        let  pass=input();
        File::create(".token").expect("Failed to create token file");
        write!(&file, "{}\n" ,teletok ).unwrap();
        write!(&file, "{}\n" ,author ).unwrap();
        write!(&file, "{}\n" ,user ).unwrap();
        write!(&file, "{}\n" ,pass ).unwrap();

    }
    else {
        if File::open("token").unwrap().metadata().unwrap().len() < 1 {
            fs::remove_file("token").unwrap();
            firstinstall();
            return;
        }
        println!("Tokens Found =>");
    }
}

fn input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer
}

fn sanitycheck() -> String {
    let mut topanic =false;
    let mut xx = Command::new("tgs2png");
    xx.arg("--help");
    xx.stdout(Stdio::null());
    let xxr = xx.output();
    match xxr {
        Err(_) => {
            println!("tgs2png is not installed on your system");
            topanic= true;
        },
        _ => ()
    }
    let mut xx = Command::new("ffmpeg");
    xx.arg("--help");
    xx.stdout(Stdio::null());
    let xxr = xx.output();
    match xxr {
        Err(_) => {
            println!("ffmpeg is not installed on your system");
            topanic= true;
        },
        _=> () 
    }
    let mut xx = Command::new("apngasm");
    xx.arg("--help");
    let xxr = xx.stdout(Stdio::null()).output();
    match xxr {
        Err(_) => {
            println!("apngasm is not installed on your system");
            topanic= true;
        },
        _ => () 
    }

    let mut pyname=String::from("python");
    let xx=  Command::new("python").arg("-m").arg("import signalstickers_client").output();
    match xx {
        Err(_) => {pyname = "python3".to_owned()},
        _ => ()
    }
    if cfg!(windows){
        pyname="python".to_owned();
    }

    let mut xx = Command::new(&pyname);
    xx.arg("-c");
    xx.arg("import signalstickers_client");
    xx.stdout(Stdio::null());
    let xxr = xx.spawn();
    match xxr {
        Err(_) => {
            println!("Please install python on your system and then using pip \nInstall signalstickers_client to manage signal upload");
            topanic= true;
        },
        _ => () 
    }

    let xx = File::open("uploader.py");
    match xx {
        Err(_) => {
            println!("Please make sure uploader.py is in Cargo's root folder or\n If you are making direct invocation to binary, make sure it's in your Present Working Directory");
            topanic=true;
        },
        _=>()
    }
    if topanic {
        panic!("Some conditions are required to be for the binaries to run, Please make sure they are met before doing so!")
    }

    return pyname;
}

fn execute_it(total : u32, name : String, auth : String , suuid : String , pass : String, binary : &String, thread : usize){
    
    let thisp = ThreadPool::new(thread);
    let binary = &binary.to_owned();
    for i in 0..total {
        let workdir ="processdir/".to_owned()+&i.to_string();
        thisp.execute(move || {    
            fs::create_dir(&workdir).expect("Can't create directories, FS error?");
            tgsup::try_decode(&(i.to_string()+".gz"), &("".to_owned()+&workdir+"/path.json"));
            println!("Starting to Convert {}.gz ", i);
            makeapng::make(&workdir, i.to_string());
        });
    }
    thisp.join();
    let file_up = File::create("upload").unwrap();
    write!(&file_up , "{}\n", &name ).unwrap();
    write!(&file_up , "{}\n", &total ).unwrap();
    tgsup::try_upload(binary, &name, &auth, &suuid, &pass, "emoji", "outputdir/", 0, total);
}

fn process_each(stickp : &String, python : &String){
    fs::remove_dir_all("outputdir");
    fs::remove_dir_all("processdir");
    fs::create_dir("outputdir").expect("Can't create outputdir, A FileSystem issue? ");
    fs::create_dir("processdir").expect("Can't create processdir, A FileSystem issue?");

    let mut token = String::from("");
    let mut auth = String::from("");
    let mut suuid = String::from("");
    let mut password = String::from("");

    let ff= File::open("token").expect("Token File must exists ");
    let mut bufr = BufReader::new(ff);
    bufr.read_line(&mut token).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut auth).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut suuid).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut password).expect("Token File is damaged delete ./token");
    let (name,total) = tgsGet::down(&token, &stickp);
    token.retain(|c| c!='\n');
    auth.retain(|c| c!='\n');
    suuid.retain(|c| c!='\n');
    password.retain(|c| c!='\n');
    execute_it(total,name , auth , suuid , password, python,  4);

}

fn do_just_upload(binary : &str){
    let mut token = String::from("");
    let mut auth = String::from("");
    let mut suuid = String::from("");
    let mut pass = String::from("");
    let mut name = String::from("");
    let mut total = String::from("");


    let ff= File::open("token").expect("Token File must exists ");
    let mut bufr = BufReader::new(ff);
    bufr.read_line(&mut token).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut auth).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut suuid).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut pass).expect("Token File is damaged delete ./token");
    let ff= File::open("upload").expect("Token File must exists ");
    let mut bufr = BufReader::new(ff);
    bufr.read_line(&mut name).expect("Token File is damaged delete ./token");
    bufr.read_line(&mut total).expect("Token File is damaged delete ./token");



    token.retain(|c| c !='\n') ;
    auth.retain(|c|  c !='\n');
    suuid.retain(|c| c !='\n');
    pass.retain(|c|  c !='\n');
    name.retain(|c|  c !='\n');
    total.retain(|c| c !='\n');
    let total = total.parse::<u32>().unwrap();
    tgsup::try_upload(binary, &name, &auth, &suuid, &pass, "emoji", "outputdir/", 0, total);
}

fn main() {
    let python = sanitycheck();
    // println!("Will be using => {}", &python);
    firstinstall();
    let to_download = false; 
    let mut up : bool = false;
    let mut hasdone =false;

    // test();
    // return;

    let args: Vec<String> = env::args().collect();
    for arg in args.into_iter().skip(1){
        hasdone=true;
        let find= arg.rfind("/");
        let mut res = arg.to_owned();
        if res.starts_with("--up") {
            up = true ;
            break;
        }
        match find {
            Some(xx) => {
                res =arg[xx+1..].to_owned();
            },
            None => ()
        }
        let ii = Instant::now();
        process_each(&res, &python);
        let oo = ii.elapsed();
        println!("This pack took {} min to get converted ",oo.as_secs()/60);
    }
    if up {
        do_just_upload(&python);
    }
    else if !hasdone {
        println!("You can also execute binary using 'cargo r --release <link> <link> <link>' ");
        println!("Please enter a Link that you want to get converted ");
        println!("You can enter link as https://t.me/addstickers/ThatIsWhatSheSaid or ThatIsWhatSheSaid , both are valid");
        let mut p = input();
        let find=p.rfind("/");
        match find {
            Some(xx) => {
                p =p[xx+1..].to_owned();
            },
            None => ()
        }
        process_each(&p,&python);
    }
}
