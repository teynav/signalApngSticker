from telegram import Update, Bot
from telegram.ext import Updater, CommandHandler, CallbackContext
import os
with open('token') as f:
    token=f.readline().split("\n")[0]
bot= Bot(token)
with open('pack') as f:
    text = f.read()
sticker_name = ""
try:
    sticker_name = text.split("/addstickers/")[1]
except:
    sticker_name = text.split("eu/stickers/")[1]
sticker_set = bot.getStickerSet(sticker_name)
ww=0
print("Downloading please wait!!")
with open("emoji", "w+") as ff:
    for i in sticker_set.stickers:
        fname = i.get_file().download()
        os.rename(fname, str(ww) + os.path.splitext(fname)[1])
        ww=ww+1
        ff.write(i.emoji[:1]+"\n")