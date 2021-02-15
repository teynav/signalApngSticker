# signalApngSticker
Convert Telegram Stickers (tgs) to Apng (&lt;300) kb for Signal
And uploads it while you grab a coffee and makes sure you have it to send it 
to friends when you are back :")

---
## Welcome :")

What do I need to run this program? Linux

If you are on Debian or Ubuntu or Arch (including their variants ) 
You __don't need to preinstall anything__, script will handle requirements when ran from
CLI for first time. You will be prompted to input password to install them.

If you want to run script by double clicking it , Install _zenity_ and _notify-send_ on your 
distribution , it's totally optional since CLI works too.

Are you on other Distributions? Get these beforehand !!!
1. gifsicle 
2. imagemagick 
3. apngasm 
4. python-pip (could be uninstalled after initial setup )
5. cmake ( could be uninstalled after initial setup )
6. (totally optional) zenity (for GUI)

What would initial setup install ? 
1. [tgs-to-gif](https://github.com/ed-asriyan/tgs-to-gif/tree/master-cpp)
2. python-telegram-bot
3. signalstickers-client 

---
V2 Intial setup 
1. Get Telegram Bot token ready
2. Now open Signal Desktop 
    - Goto Menu 
		- Toggle Developers tools 
		- On there open Console 
		- Store somehwere output of window.reduxStore.getState().items.uuid_id 
		- Also store outut of window.reduxStore.getState().items.password

#### Don't share both of these with anyone else

#### V2 of the script  handles uploading by it's own !!!!
What does v2 of this script does?

Asks you for stickerpack link of Telegram
Gives you stickerpack link of Signal :")

---

#### What if My stickerpack failed to upload? (Network issues etc)
Don't you worry, got you there. Just run the command again and it would
ask to upload packs not uploaded before due to network issues etc.

You can choose to restore/upload backups created or convert a new stickerpack to signal 

Can be done using both CLI and GUI 

Once those are uploaded they would be deleted from hard drive.

#### How to see what all packs i converted with this script ?

```
cat packs

```

#### Background 

For v2 you need just the link to stickerpack, __NO NEED TO DOWNLOAD TGS FILES__
Just have links like "https://t.me/addstickers/HalloUtya" and that's enough !!!!

For bot to upload on your behalf, using your account credentials given above.

If you want telegram bot token get it using [BotFather](https://t.me/BotFather) :")

Once Intial setup , from next time just sending link would be enough.

--- 
### Usage of v2 script

```
./script_v2_beta.sh 

./script_v2_beta.sh <filename>

Where file has list of Telegram sticker links

```

Usage of v1 of script 

```
./script_v1.sh

Converts tgs in current diretory to apng within ./outut
```
You could use it to : 

1. To create custom stickerpack from group of tgs files
2. You don't need to have telegram account , although you need source of tgs files
Dependencies of v1 of script 
1. gifsicle 
2. imagemagick 
3. apngasm 
4. tgs-to-gif 

Prefer v2 since it does job automated.

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

### With v2 porting becomes way easier since you don't have to be actively involved in porting

[Contains 80+ stickerpack as of now ported from Telegram](https://signalstickers.com/?s=author%3A%22Navneet%20Vikram%20Tey%22)


Owwww, That's nice , How can I help you
1. Suggestions , Signal +919519873721
2. Donations , [PayPal](https://www.paypal.com/paypalme/my/profile) or UPI 9519873721@ybl 
3. If you are using brave you can donate me directly.
