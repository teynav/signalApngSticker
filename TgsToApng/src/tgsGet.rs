extern crate teloxide_core;
extern crate futures;
use std::fs::File as fFile;
use std::io::{BufWriter, Write};
use tokio::fs::File;
use teloxide_core::net::Download;
use teloxide_core::{Bot, requests::Requester};
use teloxide_core::prelude::*;
async fn  download(token : &str , pack : &str) -> (String,u32){
    let thisbot=Bot::new(token);
    let down  = thisbot.get_sticker_set(pack).send().await.unwrap();
    let title = down.title;
    let stickers = down.stickers;
    let mut counter = 0;
    let mut listc = vec![];
    let  mut emojii= String::from("");
    for sticker in stickers.clone() {
        let thisbot =thisbot.clone();
        let downloading = async move {
                let mut ert = File::create(counter.to_string()+".gz").await.unwrap();
                let getf = thisbot.get_file(&sticker.file_id).send().await.unwrap().file_path;
                thisbot.download_file(&getf , &mut ert).await.unwrap();
                
                // println!("Downloading{}", counter );
                // let mut bufw = fub::new(AllowStdIo::new(ert));
                // bufw.write(&xx).await.unwrap();
        };
        listc.push(downloading);
        counter+=1;
    }
    for sticker in stickers {
        emojii.push(sticker.emoji.unwrap().chars().next().unwrap());
        emojii.push('\n');
    }
    BufWriter::new(fFile::create("emoji").unwrap()).write_all(emojii.as_bytes()).unwrap();
    futures::future::join_all(listc).await;
   (title.to_owned(),counter)
}

pub fn down(token : &str , pack : &str)-> (String, u32){
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(download(token,pack))
}