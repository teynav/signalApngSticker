from telegram import Update, Bot
from telegram.ext import Updater, CommandHandler, CallbackContext
f = open("token", "r")
token=f.readline().split("\n")[0]
bot= Bot(token)
f = open("pack", "r")
text = f.read()
sticker_name = text.split("/addstickers/")[1]
sticker_set = bot.getStickerSet(sticker_name)
ww=0
ff = open("emoji", "w+")
print("Downloading please wait!!")
for i in sticker_set.stickers:
   i.get_file().download(str(ww)+".tgs")
   ww=ww+1
   ff.write(i.emoji[:1]+"\n")
    

