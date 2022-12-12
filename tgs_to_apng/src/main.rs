use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::path::Path;
use std::process::{Command, Stdio};
use threadpool::ThreadPool;

mod ffmpeg;
mod makeapng;
mod pngtool;
mod tgs_get;
mod tgsup;
use std::time::Instant;

fn firstinstall() {
    if !Path::new("token").exists() && !Path::new("token").is_file() {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open("token")
            .unwrap();
        println!("\nTelegram Bot Token not found, \nPlease use v1 if you have tgs , \nv3 always downloads tgs requires bot token\nYou can input token now or exit");
        let teletok = input();
        println!("\nEnter default author's name");
        let author = input();
        println!("\n\nNow open Signal Desktop ,\nGoto Menu -> Toggle Developers tools -> On there open Console \nPaste output of window.reduxStore.getState().items.uuid_id");
        let user = input();
        println!(
            "You are almost there \nPaste output of window.reduxStore.getState().items.password"
        );
        let pass = input();
        File::create(".token").expect("Failed to create token file");
        write!(&file, "{}\n", teletok).unwrap();
        write!(&file, "{}\n", author).unwrap();
        write!(&file, "{}\n", user).unwrap();
        write!(&file, "{}\n", pass).unwrap();
    } else {
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
    let mut topanic = false;
    let mut xx = Command::new("tgs2png");
    xx.arg("--help");
    xx.stdout(Stdio::null());
    let xxr = xx.output();
    match xxr {
        Err(_) => {
            println!("tgs2png is not installed on your system");
            topanic = true;
        }
        _ => (),
    }
    let mut xx = Command::new("ffmpeg");
    xx.arg("--help");
    xx.stdout(Stdio::null());
    let xxr = xx.output();
    match xxr {
        Err(_) => {
            println!("ffmpeg is not installed on your system");
            topanic = true;
        }
        _ => (),
    }
    let mut xx = Command::new("apngasm");
    xx.arg("--help");
    let xxr = xx.stdout(Stdio::null()).output();
    match xxr {
        Err(_) => {
            println!("apngasm is not installed on your system");
            topanic = true;
        }
        _ => (),
    }

    let mut pyname = String::from("python");
    let xx = Command::new("python")
        .arg("-m")
        .arg("import signalstickers_client")
        .output();
    match xx {
        Err(_) => pyname = "python3".to_owned(),
        _ => (),
    }
    if cfg!(windows) {
        pyname = "python".to_owned();
    }

    let mut xx = Command::new(&pyname);
    xx.arg("-c");
    xx.arg("import signalstickers_client");
    xx.stdout(Stdio::null());
    let xxr = xx.spawn();
    match xxr {
        Err(_) => {
            println!("Please install python on your system and then using pip \nInstall signalstickers_client to manage signal upload");
            topanic = true;
        }
        _ => (),
    }

    let xx = File::open("uploader.py");
    match xx {
        Err(_) => {
            println!("Please make sure uploader.py is in Cargo's root folder or\n If you are making direct invocation to binary, make sure it's in your Present Working Directory");
            topanic = true;
        }
        _ => (),
    }
    if topanic {
        panic!("Some conditions are required to be for the binaries to run, Please make sure they are met before doing so!")
    }

    return pyname;
}

fn backup() {
    println!("Upload has failed and Hence doing backup!\n Restore with --res option");
    let backup_f = ".backupList";
    let backup_d = ".backD";
    let mut count = 0;
    let mut allback = vec![];
    if !Path::new(&backup_f).exists() || !Path::new(&backup_f).is_file() {
        fs::File::create(&backup_f).unwrap();
        fs::remove_dir_all(&backup_d).ok();
    } else {
        let ff = File::open(&backup_f).expect("backup_file Must Exist");
        let mut bufr = BufReader::new(ff);
        loop {
            let mut otherback = String::from("");
            let status = bufr.read_line(&mut otherback);
            match status {
                Err(_) => break,
                Ok(x) => {
                    if x == 0 {
                        break;
                    }
                }
            }
            otherback.retain(|c| c != '\n');
            let num = otherback.parse::<i32>();
            let mut finalnum = 0;
            match num {
                Err(_) => {
                    println!("Backup File Corrupted ={}= ", otherback);
                }
                Ok(a) => finalnum = a,
            }
            allback.push(finalnum);
        }
        //        println!("{:?}" , &allback);
        while allback.contains(&count) {
            count = count + 1;
        }
    }
    fs::create_dir_all(format!("{}/{}", &backup_d, &count)).ok();
    fs::rename("upload", format!("{}/{}/upload", &backup_d, &count)).ok();
    fs::rename("emoji", format!("{}/{}/emoji", &backup_d, &count)).ok();
    fs::rename("outputdir", format!("{}/{}/outputdir", &backup_d, &count)).ok();
    let tmm = OpenOptions::new()
        .append(true)
        .read(false)
        .write(true)
        .open(&backup_f)
        .unwrap();
    let mut tmm = BufWriter::new(tmm);
    write!(tmm, "{}\n", count).ok();
}

fn restore(binary: &str) {
    fs::remove_dir_all("outputdir").ok();
    fs::remove_file("emoji").ok();
    fs::remove_file("upload").ok();
    let backup_f = ".backupList";
    let backup_d = ".backD";
    let mut allback = vec![];
    if !Path::new(&backup_f).exists()
        || !Path::new(&backup_f).is_file()
        || Path::new(&backup_d).is_dir()
    {
        ()
    }
    let ff = File::open(&backup_f).expect("BackupFile Must Exist");
    let mut bufr = BufReader::new(ff);
    loop {
        let mut otherback = String::from("");
        let status = bufr.read_line(&mut otherback);
        match status {
            Err(_) => break,
            Ok(x) => {
                if x == 0 {
                    break;
                }
            }
        }
        otherback.retain(|c| c != '\n');
        let num = otherback.parse::<i32>();
        let mut finalnum = 0;
        match num {
            Err(_) => println!("Backup File Corrupted"),
            Ok(a) => finalnum = a,
        }
        allback.push(finalnum);
    }
    for count in allback.clone() {
        fs::rename(format!("{}/{}/upload", &backup_d, &count), "upload").ok();
        fs::rename(format!("{}/{}/emoji", &backup_d, &count), "emoji").ok();
        fs::rename(format!("{}/{}/outputdir", &backup_d, &count), "outputdir").ok();
        do_just_upload(binary);
        fs::remove_dir_all(format!("{}/{}", &backup_d, &count)).unwrap();
        fs::remove_dir_all("outputdir").ok();
        fs::remove_file("emoji").ok();
        fs::remove_file("upload").ok();
    }
    let mut allback2 = vec![];
    loop {
        let mut otherback = String::from("");
        let status = bufr.read_line(&mut otherback);
        match status {
            Err(_) => break,
            Ok(x) => {
                if x == 0 {
                    break;
                }
            }
        }
        otherback.retain(|c| c != '\n');
        let num = otherback.parse::<i32>();
        let mut finalnum = 0;
        match num {
            Err(_) => println!("Backup File Corrupted"),
            Ok(a) => finalnum = a,
        }
        allback2.push(finalnum);
    }
    for thisf in allback {
        allback2.retain(|&x| x != thisf);
    }
    fs::remove_file(&backup_f).ok();
    let finalfile = OpenOptions::new()
        .write(true)
        .create(true)
        .open(backup_f)
        .unwrap();
    let mut bufw = BufWriter::new(finalfile);
    for allcont in allback2 {
        write!(bufw, "{}\n", allcont).ok();
    }
}

fn execute_it(
    total: u32,
    name: String,
    auth: String,
    suuid: String,
    pass: String,
    binary: &String,
    thread: usize,
    is_video: bool,
) {
    let thisp = ThreadPool::new(thread);
    let binary = &binary.to_owned();
    for i in 0..total {
        let workdir = "processdir/".to_owned() + &i.to_string();
        thisp.execute(move || {
            fs::create_dir(&workdir).expect("Can't create directories, FS error?");
            let mut extension = ".gz";
            if !is_video {
                tgsup::try_decode(
                    &(i.to_string() + ".gz"),
                    &("".to_owned() + &workdir + "/path.json"),
                );
                println!("Starting to Convert {}.gz ", i);
            } else {
                println!("Starting to Convert {}.webm",&i);
                extension = ".webm";
            }
            makeapng::make(&workdir, i.to_string(), is_video, i.to_string()+".webm");
            fs::remove_file(i.to_string() + &extension);
        });
    }
    thisp.join();
    let file_up = File::create("upload").unwrap();
    write!(&file_up, "{}\n", &name).unwrap();
    write!(&file_up, "{}\n", &total).unwrap();
    let isdone = tgsup::try_upload(
        binary,
        &name,
        &auth,
        &suuid,
        &pass,
        "emoji",
        "outputdir/",
        0,
        total,
    );
    if !isdone {
        //panic!("Couldn't upload the stickerpack!");
        backup();
    }
    // clear("gz");
}

// Don't need it
// fn clear(s: &str) {
//     let currentdir = env::current_dir().unwrap();
//     let to_remove_at = fs::read_dir(currentdir.as_path()).unwrap();
//     to_remove_at.for_each(|content| {
//         let file = content.unwrap();
//         let type_f = file.file_type().unwrap();
//         if type_f.is_dir() {
//             return;
//         }
//         match file.path().extension() {
//             Some(part) => {
//                 if part.eq(s) {
//                     let filename = file.file_name();
//                     fs::remove_file(filename).ok();
//                 }
//             }
//             None => return,
//         }
//     });
// }
fn process_each(stickp: &String, python: &String) {
    fs::remove_dir_all("outputdir").ok();
    fs::remove_dir_all("processdir").ok();
    fs::create_dir("outputdir").expect("Can't create outputdir, A FileSystem issue? ");
    fs::create_dir("processdir").expect("Can't create processdir, A FileSystem issue?");

    let mut token = String::from("");
    let mut auth = String::from("");
    let mut suuid = String::from("");
    let mut password = String::from("");

    let ff = File::open("token").expect("Token File must exists ");
    let mut bufr = BufReader::new(ff);
    bufr.read_line(&mut token)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut auth)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut suuid)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut password)
        .expect("Token File is damaged delete ./token");
    let stuff = tgs_get::down(&token, &stickp);
    let name = stuff.0;
    let total = stuff.1;
    let is_video = stuff.2;
    token.retain(|c| c != '\n');
    auth.retain(|c| c != '\n');
    suuid.retain(|c| c != '\n');
    password.retain(|c| c != '\n');
    execute_it(total, name, auth, suuid, password, python, 4, is_video);
}

fn do_just_upload(binary: &str) {
    let mut token = String::from("");
    let mut auth = String::from("");
    let mut suuid = String::from("");
    let mut pass = String::from("");
    let mut name = String::from("");
    let mut total = String::from("");

    let ff = File::open("token").expect("Token File must exists ");
    let mut bufr = BufReader::new(ff);
    bufr.read_line(&mut token)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut auth)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut suuid)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut pass)
        .expect("Token File is damaged delete ./token");
    let ff = File::open("upload").expect("Token File must exists ");
    let mut bufr = BufReader::new(ff);
    bufr.read_line(&mut name)
        .expect("Token File is damaged delete ./token");
    bufr.read_line(&mut total)
        .expect("Token File is damaged delete ./token");

    token.retain(|c| c != '\n');
    auth.retain(|c| c != '\n');
    suuid.retain(|c| c != '\n');
    pass.retain(|c| c != '\n');
    name.retain(|c| c != '\n');
    total.retain(|c| c != '\n');
    let total = total.parse::<u32>().unwrap();
    let isdone = tgsup::try_upload(
        binary,
        &name,
        &auth,
        &suuid,
        &pass,
        "emoji",
        "outputdir/",
        0,
        total,
    );
    if !isdone {
        //  panic!("Couldn't upload the stickerpack");
        backup();
    }
}

fn test() -> bool {
    let path = std::env::current_exe().unwrap();
    let path = path.to_str().unwrap();
    if path.contains("/target/release/tgs_to_apng") || path.contains("/target/debug/tgs_to_apng") {
        println!("\n Path for binary is {} \n \n Are you running cargo r --release or cargo r??? \n Another way to do it is by doing \n \n cargo install path . \n Notice \".\" at the end\n \n You can find installed binary at ~/.cargo/bin/tgs_to_apng\n You won't be able to directly execute it because your PRESENT WORKING DIRECTORY must have \"uploader.py\", Which is present inside this folder of the repo\n\n Example \n cargo install --path . \n mkdir ~/stickerdir \n cp uploader.py ~/stickerdir/ \n cd ~/stickerdr \n ~/.cargo/bin/tgs_to_apng \n\n You can also add ~/.cargo/bin to your $PATH\n\n", path);

        return false;
    }
    true
}
fn main() {
    let python = sanitycheck();
    // println!("Will be using => {}", &python);
    firstinstall();
    let mut up: bool = false;
    let mut hasdone = false;
    let bint = test();
    let mut file_list = vec![];
    let args: Vec<String> = env::args().collect();
    for arg in args.into_iter().skip(1) {
        hasdone = true;
        let find = arg.rfind("/");
        let mut res = arg.to_owned();
        if res.eq("-") {
            continue;
        }
        if res.starts_with("--up") {
            up = true;
            break;
        }
        if res.starts_with("--res") {
            restore(&python);
        }
        match find {
            Some(xx) => {
                res = arg[xx + 1..].to_owned();
            }
            None => (),
        }
        if Path::new(&res).exists() {
            file_list.push(res.clone());
            continue;
        }
        println!("{}", &arg);
        let ii = Instant::now();
        process_each(&res, &python);
        let oo = ii.elapsed();
        println!("This pack took {} min to get converted ", oo.as_secs() / 60);
    }
    for file_d in file_list {
        println!("Discovered File {}", &file_d);
        let file_temp = OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .open(&file_d)
            .unwrap();
        let reader = BufReader::new(file_temp);
        for line in reader.lines() {
            hasdone = true;
            let mut theline = line.unwrap();
            println!("Working on Pack {}", &theline);
            let find = theline.rfind("/");
            match find {
                Some(xx) => {
                    theline = theline[xx + 1..].to_owned();
                }
                None => (),
            }
            let ii = Instant::now();
            process_each(&theline, &python);
            let oo = ii.elapsed();
            println!("This pack took {} min to get converted ", oo.as_secs() / 60);
        }
    }
    if up {
        do_just_upload(&python);
    } else if !hasdone {
        if bint {
            println!(
                "You can also execute binary using '~/.cargo/bin/tgs_to_apng <filename> <link>' "
            );
        } else {
            println!("You can also execute binary using 'cargo r --release <filename> <link> ' ");
        }
        println!("Any number of files and links can be provided");
        println!("You can enter link as https://t.me/addstickers/ThatIsWhatSheSaid or ThatIsWhatSheSaid , both are valid");
        println!("File which is given as input must have list of all links in it ");
        let mut p = input();
        let find = p.rfind("/");
        match find {
            Some(xx) => {
                p = p[xx + 1..].to_owned();
            }
            None => (),
        }
        let ii = Instant::now();
        process_each(&p, &python);
        let oo = ii.elapsed();
        println!("This pack took {} min to get converted ", oo.as_secs() / 60);
    }
}
