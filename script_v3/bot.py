import os
import anyio
from signalstickers_client import StickersClient
from signalstickers_client.models import LocalStickerPack, Sticker
from telegram import Update, Bot
from telegram.ext import Updater, CommandHandler, CallbackContext
async def main():
    def add_sticker(path, emoji):

        stick = Sticker()
        stick.id = pack.nb_stickers
        stick.emoji = emoji

        with open(path, "rb") as f_in:
            stick.image_data = f_in.read()

        pack._addsticker(stick)

    pack = LocalStickerPack()
    f = open("token", "r")
    token=f.readline().split("\n")[0]
    author=f.readline().split("\n")[0]
    uuid= f.readline().split("\n")[0]
    password =f.readline().split("\n")[0]
    pack.author = author
    bot= Bot(token)
    f = open("pack", "r")
    text = f.read()
    
    sticker_name = ""
    try:
       sticker_name = text.split("/addstickers/")[1]
    except:
       sticker_name = text.split("eu/stickers/")[1]
    sticker_set = bot.getStickerSet(sticker_name)
    pack.title = sticker_set.name
    ww=0
    emoji = open("emoji" , "r")
    for i in sticker_set.stickers:
       emoji_i = emoji.readline().split("\n")[0]
       file = "result/"+str(ww)+".png"
       add_sticker(file , emoji_i )
#      print (file + " corresponds to " + emoji_i)
       ww=ww+1
    # Specifying a cover is optional
    # Instanciate the client with your Signal crendentials
    print("-->")
    print("Final stage Uploading is gonna start now!!!!!")
    print ("token  ="+token )
    print ("author ="+author )
    print ("uuid (DO NOT SHARE)="+uuid)
    print ("password (DO NOT SHARE)="+password)
    print ("Pack name = "+ pack.title )
    print ("-->")
    ff=open("packs","a+")
    async with StickersClient(uuid,password) as client:
         pack_id, pack_key = await client.upload_pack(pack)
    print("Pack uploaded!\n\nhttps://signal.art/addstickers/#pack_id={}&pack_key={}".format(pack_id, pack_key))
    ff.write("### Pack = "+pack.title+"\n\nhttps://signal.art/addstickers/#pack_id={}&pack_key={}".format(pack_id, pack_key)+"\n---\n")
    ff.close() 
if __name__ == '__main__':
    anyio.run(main)


