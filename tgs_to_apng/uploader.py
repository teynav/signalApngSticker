#!/usr/bin/python
import os
import anyio
import sys
from signalstickers_client import StickersClient
from signalstickers_client.models import LocalStickerPack, Sticker
async def main():
    def add_sticker(path, emoji):

        stick = Sticker()
        stick.id = pack.nb_stickers
        stick.emoji = emoji

        with open(path, "rb") as f_in:
            stick.image_data = f_in.read()

        pack._addsticker(stick)

    pack = LocalStickerPack()
    pack.title = sys.argv[1]
    pack.author=sys.argv[2]
    uuid= sys.argv[3]
    password = sys.argv[4]
    filemoji = sys.argv[5]
    location = sys.argv[6]
    start = int(sys.argv[7])
    end = int(sys.argv[8])
    emoji = open(filemoji , "r")
    emojidata=emoji.readlines()
   # print(pack.title,pack.author,uuid,password,filemoji,location,start,emoji)
    ww=start
    while ww < end:
       emoji_i = emojidata[ww].split("\n")[0]
       file = location+str(ww)+".png"
       add_sticker(file , emoji_i )
       print (file + " corresponds to " + emoji_i)
       ww=ww+1
    # Specifying a cover is optional
    # Instanciate the client with your Signal crendentials
    print("-->") 
    print("uuid ="+uuid+"")
    print("password ="+password)
    print("-->")
    print("Final stage Uploading is gonna start now!!!!!")
    print ("author ="+pack.author )
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


