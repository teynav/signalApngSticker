# signalApngSticker
Convert Telegram Stickers (tgs) to Apng (&lt;300) kb for Signal

---
This is my first github repo, do give me suggestions!!
---
What do I need to run this program? Linux
What do I need to install beforehand? Good question, Here's the list.
1. gifsicle 
2. imagemagick 
3. apngasm 
4. tgs-to-gif 
You can easily find these in your repo or AUR 


## V2 of the script is handles uploading by it's own !!!!
What does v2 of this script does?

Does all mentioned tasks of v1 , and then make stickerpack , uploads it
Give you stickerpack link :")

For v2 you need just the link to stickerpack, NO NEED TO DOWNLOAD TGS FILES
Just have "https://t.me/addstickers/HalloUtya" and that's enough !!!!

While setting up v2 for first time you might be asked some info for stickerpack-script
for bot to upload on your behalf.

Also you will need to setup a telegram bot using t.me/BotFather and get token of it :")

Once done, from next time just sending link would be enough.


What do this script do? (Just v1 )
1. Convert tgs to gif
2. optimize that gif
3. breaks gif into frames
4. Make apng out of those frames



How do i do this?
1. Install above mentioned packages
2. Copy your tgs into a folder
3. Copy this script to same folder
4. Run it!!
5. Wait for cpu to cook your apngs
6. Look for all apngs in ./output/
7. Create stickerpack using those apngs all are below 300kb !!!

You say so, but what have you done??? 
(some of these were done using giff-older which used gif from @StickerDownloadBot of telegram it had artifacts, newer script.sh solves this issue where there were artifacts)
1. https://signal.art/addstickers/#pack_id=d511843bf15edcc2ed58354432dcaf56&pack_key=71e7d65cccc31ac6f6ca68f8ee0942e9f668ae4f62ce156e1fa6fdfe3a7c825f
2. https://signal.art/addstickers/#pack_id=a92ed43943b90b6bed0f014a3d0a987d&pack_key=e140abc992215c45c0119e6cb16435aab536a3c703f4ace422f9ec09c2ee4edc
3. https://signal.art/addstickers/#pack_id=8335949df80c37eb06c7c7872a75af68&pack_key=f67bd792a3a1ac00be64b6b6d21274c1e50ffbd66a52504345c1139cf8ad8a78
4. https://signal.art/addstickers/#pack_id=8f09e9096b3b4584fe7af62269854227&pack_key=dd35359072bbd3e9280d977204b8218fc34446c9aa5e5d3b4793e4fab39da44c
5. https://signal.art/addstickers/#pack_id=0876b357a5a8b06a192506aee2f2e09e&pack_key=e6fa8c3db15d29c3a42e78e2f246bfedb39c8bf0034b067fd6bf5c2289f3e6b1
6. https://signal.art/addstickers/#pack_id=4d14ed77727bb3a53f379064c12a3ec5&pack_key=a69794346f71ecfc533a4c86efda25167e433547a48f40112e091b48874e0a60
7. https://signal.art/addstickers/#pack_id=249818aaf61019027e67397e6af41c4f&pack_key=1f42ecfc4162e5b90a176911967ce6aa30e00d996b6babe72b2172122df25e02
8. https://signal.art/addstickers/#pack_id=82bbb0e722f3cfbfa73c981567c8f52b&pack_key=ff01047867757bebef2d6b3eea0cdca5da294013dc38433bf6d4e65064e11d24
9. https://signal.art/addstickers/#pack_id=293f5ffc19efa5ec5e777c5de95929d2&pack_key=ea0f54e119172bddd959469f4d725c9ee37a43edfc310c49a28067de656bc800
10. https://signal.art/addstickers/#pack_id=86f00a7bff29a7869b1ca318566ab745&pack_key=31ef41ec65c2606e20e091c662e4e3d2743ba63f2af186e23ad971d53c0ec7cd
------mostly newwer from here------
11. https://signal.art/addstickers/#pack_id=a1a672ac3fa05e65f883ec4fbc1d225d&pack_key=6efdf440fe0c393764856718688c85056be14305e9174f8a686446d9375726e5
12. https://signal.art/addstickers/#pack_id=14945401b315dfbbf9798aaa64576fea&pack_key=2cd2418d3fa96d360297b1b866afc21d90847a16fc8d642f27a1dbc288c75af6
13. https://signal.art/addstickers/#pack_id=f8631c82dc81a9b24725dc94fd169670&pack_key=031bcf6c2922c6e954fa1a49acdc534b0111c15a8018bfea6cc72711ccf1c2bc
14. https://signal.art/addstickers/#pack_id=dccbf815d26a880158f1083bbf3460a1&pack_key=cde84211529be69fb1f0932c90213a509f2511eca7c06e089c3f24f04c0095de
15. https://signal.art/addstickers/#pack_id=e3265a7f97f53b340cc6010f6ac01db8&pack_key=c4255477343a089f52dd880c79cbdd6b7495004b244c10469f94e40106496437
16. https://signal.art/addstickers/#pack_id=6514feedeffde5017caca3fa4c86bd12&pack_key=493eefa127ad0f6980dd48374941dc7e0a4f666fa8cae4a11ac5b85f8cdd5a8d
17. https://signal.art/addstickers/#pack_id=fd8c232fc71476fad14ee9700d07cb18&pack_key=94fd6ea0b35dd49dbc202825d42f4eef8174e88c6e2c1a55d1effef4bf9bf6b4
18. https://signal.art/addstickers/#pack_id=d1da8a723d5a05c1604b3a76bce0c193&pack_key=2703f6d169020798da1f88e45e77755016b7fab3f8d3246fdec4b8d5c3f75e40
19. https://signal.art/addstickers/#pack_id=37c23bb7dcd467d98aee9ca76e1b3313&pack_key=98cb227aa500af6db6c8e7559883f25b0a9b24738cc4b410728ef10955f2d9f0
20. https://signal.art/addstickers/#pack_id=b94f064a0370cc90c26b1fca73fd3eee&pack_key=56d26b9060217e8eb25eb8bba33597520bede80d85e80f42bd31e729dd29c2a1
21. https://signal.art/addstickers/#pack_id=70ce21c56d10da385a2d4cbd01b0a598&pack_key=a6720ed69612df5ddf7743c070d99916a34679039701eacb82d28f9587a208ae
22. https://signal.art/addstickers/#pack_id=05cbed4d89e849f74b03757927ae73a0&pack_key=eb8aa8f1080fbcd30b25d0dd4b001dbc91ce5516830cc2fadac96fda2bd4df50
23. https://signal.art/addstickers/#pack_id=35e7882efaeb30bb8e289f00603f731c&pack_key=dcc9d56d642a18b252923a9f15b301e601f44c2ef61c714e1d4bd2f131f3a406
24. https://signal.art/addstickers/#pack_id=c33434436cb10a37963a9542fcf9b893&pack_key=11d39a4a28f739fd4ac2a0d804072e0fbb6ae3d549f98ad8d37fdb520d3c979b
25. https://signal.art/addstickers/#pack_id=31c290aff32928b5b161b0d9fb1097d0&pack_key=6f989aecedfedadd1f76873cc5e4281e9772b891bbde45990eeb5a20be38ccf5
26. https://signal.art/addstickers/#pack_id=51267505a33cee27471fc3c20dbe60ab&pack_key=d28d2e36221433fb3de105b387e4d9b94e7851b3f375945a3981d9bb4d1a5eb6
27. https://signal.art/addstickers/#pack_id=ffea53554747992cbdb08af7a4d8503d&pack_key=32c5553fb840f4d3cc79a5d226f6ae463d0c0cf229b46b3c519c820e55344772
28. https://signal.art/addstickers/#pack_id=05108c1b49e3bfb44b5fefc977a7b092&pack_key=03f18c89c7cde5e20fb1fda5253035d17fd20a0ba928aa5a4aa7d4652ea79cab
29. https://signal.art/addstickers/#pack_id=84fd191e08c23c4f6006b9c79b5527b8&pack_key=42d738a1997235b379e9cd790543ff7050ffeb272b525c4f1d5cddf253e427ae
30. https://signal.art/addstickers/#pack_id=106e3684cdf352fa75e53031ac632e75&pack_key=f86ab9a41679a2cf854080227e5f5f47c722162a8597b9a65b94becac2dcf88e
31. https://signal.art/addstickers/#pack_id=b2e36679c553fb53afea37fc9e0d2a72&pack_key=77ce98a2842bbbe6bf14cf5d372c2f950bd57d0c3119aa2024166dbd918eb303
32. https://signal.art/addstickers/#pack_id=4b476385d6d7e99124f66760c6937252&pack_key=736214db826620abce430fde7adff0c1af1a97bbd2c58ce45961ee5dc31e0228
33. https://signal.art/addstickers/#pack_id=e245c4a3c57afed5b182c3b272168466&pack_key=668ed691cebb200023aba746fd5f843f25dd39d1bea420444ff4d48f4ddaf148
34. https://signal.art/addstickers/#pack_id=61ce6e6519c799cb176e7c9f6f447152&pack_key=7286abad6b45102983a9cdd86605b74a6e6a9e6a31fd188b00d4d2926021e144
35. https://signal.art/addstickers/#pack_id=64b2d9e14c0761d49c41b235c528a0be&pack_key=e29b28e7fe2d70341f93bed8b7b7b2606c7e771ae5084f71c3674303627e6200
36. https://signal.art/addstickers/#pack_id=ca20cbf581f00d3f88437896bc6ba6c4&pack_key=9ce4c2aac20bc07f84f526a13076fe50dfe346098909a7698993f504df846d6d
37. https://signal.art/addstickers/#pack_id=d9972b04ad521cf093a92b4a5287a93e&pack_key=5f082cd8ed158672001c16fcb7e0e0f3df67f15220c9c82fce2a6f4646674b62
38. https://signal.art/addstickers/#pack_id=c922b1a77f750817e0ff84c8d2de8f36&pack_key=558a7c30519c469a84deddefedbc2a890750092a878816539255587106ae41af
39. https://signal.art/addstickers/#pack_id=6e38b26e1dd6f2cc28b25e7c70716326&pack_key=65c293f7c55d89be69da419f454a2eabf08a40f4e8e17816be9cca101e062e10
40. https://signal.art/addstickers/#pack_id=fece39722eba4ddd1460ee34a79a7bd9&pack_key=28e0c190f5c59e9546ba3873e139a55cef650a98ee73c62b16515cff84e328e6
41. https://signal.art/addstickers/#pack_id=7782f672451c6701238a2cf9d0c64fcf&pack_key=37a2b8941f0a3b64fa364eea4ebf4a04d71794aed33cd03040acf4db2751f94f
42. https://signal.art/addstickers/#pack_id=b93f2a708776ba503957199e35750452&pack_key=457fc7c2e13b76dcca8458dc985e699bf9bf0a33e4ce4b668288b49dca9f09a5
43. https://signal.art/addstickers/#pack_id=2c117ca27e01ca6ebb53d763c1ccbc2c&pack_key=6cddfcf981b1600bf352daedace2e086dccdd1ed486ccddfac1dd9feabd9bdf7
44. https://signal.art/addstickers/#pack_id=81d97fe0d2c67b00ef47bfe134f1055d&pack_key=c90e22289e48760843552c48146abd57ecc3851474396207ecad9562cfe7c9ca
45. https://signal.art/addstickers/#pack_id=c3418a1c15c96da1be3271c154f82177&pack_key=26a268510c5ff1858d50a1ddf80dea862c45d00797c77e86be315f488fad3da2
46. https://signal.art/addstickers/#pack_id=f679a710be51d240fc75e64ac10aef6b&pack_key=ae18be70c80e2fdedef90e95d5007ba9fa8fb86ec4f1cc1913053578d16888cf
47. https://signal.art/addstickers/#pack_id=2c117ca27e01ca6ebb53d763c1ccbc2c&pack_key=6cddfcf981b1600bf352daedace2e086dccdd1ed486ccddfac1dd9feabd9bdf7
48. https://signal.art/addstickers/#pack_id=a1c5cb861e12005b3371ae461f4d43b4&pack_key=a2adf5181b5676aae3f3ac2c376f7650d7eebbeab475eb25c3ee9cea9eaa4aac
49. https://signal.art/addstickers/#pack_id=e0be187724e258c2236f493ac1744f04&pack_key=6ba0dcee8281f30de65885d5a2ae5ac6b9e564c8ba819dd76e690cd7f25f1352
50. https://signal.art/addstickers/#pack_id=77cc8f2b489e0f70ddd7466326d78251&pack_key=85c6c6e0b429b0ddb1f01a479dc97b9bda046f6f724137c4d64a3e836c53f9ff

Owwww, That's nice , How can I help you
1. Suggestions , Signal +919519873721
2. Donations , [PayPal](https://www.paypal.com/paypalme/my/profile) or UPI 9519873721@ybl 
3. If you are using brave you can donate me directly.
