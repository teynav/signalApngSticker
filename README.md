# signalApngSticker
Convert Telegram Stickers (tgs) to Apng (&lt;300) kb for Signal
And uploads it while you grab a coffee and makes sure you have it to send it 
to friends when you are back :")

---
## Welcome :")

What do I need to run this program? Linux

What do I need to install beforehand? Good question, Here's the list.
1. gifsicle 
2. imagemagick 
3. apngasm 
4. [tgs-to-gif](https://github.com/ed-asriyan/tgs-to-gif/tree/master-cpp)
5. zsh (soon to be replaced with bash)
6. sudo pip install python-telegram-bot ( For v2 only )
7. sudo pip install signalstickers-client ( For v2 only )

---
You can easily find these in your repo or AUR 
With v2 you will be able to download these using script except for tgs-to-gif 
For python requirements run pip commands 

---
### Usage of v2 script

```
./script_v2_beta.sh 

./script_v2_beta.sh <filename>

Where file has list of Telegram sticker links

```
## V2 of the script is handles uploading by it's own !!!!
What does v2 of this script does?

Does all mentioned tasks of v1 , and then make stickerpack , uploads it
Give you stickerpack link :")

For v2 you need just the link to stickerpack, NO NEED TO DOWNLOAD TGS FILES
Just have "https://t.me/addstickers/HalloUtya" and that's enough !!!!

While setting up v2 for first time you might be asked some info for signalstickerpack-client script
for bot to upload on your behalf, using your account credentials.

Also you will need to setup a telegram bot using [BotFather](https://t.me/BotFather) and get token of it :")

Once done, from next time just sending link would be enough.

---

What do this script do? (Just v1 )
1. Convert tgs to gif
2. optimize that gif
3. breaks gif into frames
4. Make apng out of those frames

How do i do this? (For v2 )
1. Install above mentioned packages and python modules 
2. Copy script_v2 , bot.py , download.py to a folder. 
3. For first time you will be asked bot token and authentican info from Signal-Desktop
4. Do you have link to Telegram-Sticker ? Enter it when prompted eg : https://t.me/addstickers/MsWitchCat
5. Grab a coffee , Because you will now get link to signalstickerpack after wizardry is done !!!!

How do i do this? (For v1 )
1. Install above mentioned packages
2. Copy your tgs into a folder
3. Copy this script_v1  to same folder
4. Run it!!
5. Wait for cpu to cook your apngs
6. Look for all apngs in ./output/
7. Create stickerpack using those apngs all are below 300kb !!!

You say so, but what have you done??? 
(some of these were done using giff-older which used gif from @StickerDownloadBot of telegram it had artifacts, newer script_v1.sh solves this issue where there were artifacts)

### With v2 porting becomes way easier since you don't have to be actively involved in porting

[Contains 60+ stickerpack as of now ported from Telegram](https://signalstickers.com/?s=author%3A%22Navneet%20Vikram%20Tey%22)


Owwww, That's nice , How can I help you
1. Suggestions , Signal +919519873721
2. Donations , [PayPal](https://www.paypal.com/paypalme/my/profile) or UPI 9519873721@ybl 
3. If you are using brave you can donate me directly.
