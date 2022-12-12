extern crate futures;
extern crate teloxide;
use std::fs::File as fFile;
use std::io::{BufWriter, Write};
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::{requests::Requester, Bot};
use tokio::fs::File;
async fn download(token: &str, pack: &str) -> (String, u32, bool) {
    let thisbot = Bot::new(token);
    let down = thisbot.get_sticker_set(pack).send().await.unwrap();
    let title = down.title;
    let is_video = down.format == teloxide::types::StickerFormat::Video;
    let stickers = down.stickers;
    let mut counter = 0;
    let mut extension = ".gz";
    if is_video {
        extension = ".webm";
    }
    let mut listc = vec![];
    let mut emojii = String::from("");
    for sticker in stickers.clone() {
        let thisbot = thisbot.clone();
        let downloading = async move {
            let mut ert = File::create(counter.to_string() + &extension)
                .await
                .unwrap();
            let getf = thisbot
                .get_file(&sticker.file.id)
                .send()
                .await
                .unwrap()
                .path;
            thisbot.download_file(&getf, &mut ert).await.unwrap();

            // println!("Downloading{}", counter );
            // let mut bufw = fub::new(AllowStdIo::new(ert));
            // bufw.write(&xx).await.unwrap();
        };
        listc.push(downloading);
        counter += 1;
    }
    for sticker in stickers {
        emojii.push(sticker.emoji.unwrap().chars().next().unwrap());
        emojii.push('\n');
    }
    BufWriter::new(fFile::create("emoji").unwrap())
        .write_all(emojii.as_bytes())
        .unwrap();
    futures::future::join_all(listc).await;
    (title.to_owned(), counter, is_video)
}

pub fn down(token: &str, pack: &str) -> (String, u32, bool) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(download(token, pack))
}
