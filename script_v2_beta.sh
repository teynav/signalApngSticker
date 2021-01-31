#!/bin/zsh

# This is function I used to keep log instead of echo since you can comment the line #1 in the function and there would be no log
# This is easier than editing every other echo and removing it sometimes.
log()
{ 
	echo $1
}

#This function removes some frames depending on total size of resultant png
#If it's more than 700kb (look at "workplz") then $1 is 1 , and frames removed are 
# 		a=(2 5 7 8 11 13 14 17 20 22 26 30)
#After it removes those frames It reorders files in order
#For example
#Files : 
# filepng01.png
# filepng02.png
# filepng03.png
# 
# You removed 2nd file , Now files become
# filepng01.png
# filepng03.png
#
#apnggasm doesn't work for files not in sequence as here , Hence this function ensures it's in sequence
#after removing filepng02, it makes filepng03 become filepng02.

remove()
{
	ref=("01" "02" "03" "04" "05" "06" "07" "08" "09" "10" "11" "12" "13" "14" "15" "16" "17" "18" "19" "20" "21" "22" "23" "24" "25" "26" "27" "28" "29" "30")
	let "count=0"
	a=(1)
	if [ $1 -eq "1" ]
	then
		a=(2 5 7 8 11 13 14 17 20 22 26 30)
	elif [ $1 -eq "2" ]
	then
	        a=(1 3 7 11 13 17 20 24 29)
	elif [ $1 -eq "3" ]
	then
		a=(3 9 12 15 20 24)
	elif [ $1 -eq "4" ]
	then
		a=(4 8 12 14)
	else
		a=(5 10)
	fi
	log $1
	let "val=1"
        for file in ./filepng*.png
	do
		let "got=0"
              for i in $a
              do
	     # log "checking $file with index $i at index $val"
	           if [ $val -eq $i ]
	            then
			got=1
		        #log "removing $file at index $i" 
			rm $file 
			break 
	           fi
              done
	if [ $got -eq "0" ]
	then
		let "count=$count+1"
		mv $file "filepng"${ref[$count]}".png"
	#	log "${ref[$count]}"
		
	fi
        let "val=$val+1"
        done
}

# Depending on file size decide how many frames to drop and call "remove" for it
# Then collect all pngs left after dropping frames to create new file and check for it's size.
# if it's <300 kb then it's alright!!!!!
# otherwise redo the process again :"(

workplz()
{
	a=$(du "$1" | sed -e "s/\s.*png//")
	if [ $a -gt "700" ]
	then
		remove 1
	elif [ $a -gt "600" ]
	then
		remove 2
	elif [ $a -gt "500" ]
	then
		remove 3
	elif [ $a -gt "400" ]
	then
		remove 4
	elif [ $a -gt "300" ]
	then
		remove 5
	else
		echo "$1 already at best"
		if [ ! -f tmp.png ]
		then
			mv $1 tmp.png 
		fi
		return
	fi
	rm tmp.png  
	apngasm tmp.png $(ls filepng* | tr '\n' ' ') -z0 
        workplz tmp.png 
}
INPUU=""
NOTIFY="NO?"
post () {
#	notify-send "In post with $1 and $(tty)"
if [[ "$(tty)" == "not a tty" ]] ;
then
	  NOTIFY=""
		if [ ! -f /usr/bin/zenity ] ;
		then
			notify-send "Please install zenity"
			exit 1
		fi
		INPUU=$(zenity --entry --title="Sticker Pack Creator <3" --text="$1")
		if [[ "$INPUU" == "" ]] ;
		then
			notify-send "Script cancelled"
			exit 
		fi 
else
		echo $1
		read INPUU
fi
}
#Output comes here

mkdir output 
#install dep
depi () {
	if [ -f /usr/bin/apt ]
	then 
		log "Installing $1 for ubuntu"
		if sudo apt install $1  ; 
		then
			log "installed" 
		else
			exit 
		fi 
	elif [ -f /usr/bin/pacman ] ;
	then
		log "Installing $1 for arch" 
     if sudo pacman -S $1  ; 
		then
			log "installed" 
		else
			exit 
		fi
	else
		log "Install $1 manually for now"
	fi
}


#Check and ask to install dep 
installdep () {
	if [ -f /usr/bin/apngasm ] ;
	then
		log "apngasm is installed !!!!!!!!!!!!!"
	else
		log "Trying to install apnggasm"
		depi apngasm  
	fi 
  
	if [ -f /usr/bin/gifsicle ] ;
	then
		log "Gifsicle is installed !!!!!!!!!!!!"
	else
		log "Trying to install gifsicle"
		depi gifsicle 
	fi 

	if [ -f /usr/bin/convert ] ;
	then
		log "Imagemagick is installed !!!!!!!!"
	else
		log "Imagemagick is not installed, Trying to install it"
		depi imagemagick 
	fi 
		
	if [ -f /usr/bin/tgs-to-gif ] ;
	then
		log "tgs-to-gif is installed !!!!!"
	else 
		log "Not installed tgs to gif , Need to  take help of github here " 
    log "visit https://github.com/ed-asriyan/tgs-to-gif/tree/master-cpp" 
	  exit 1 
	fi 

} 

maininstall() {
for file in ./*.tgs 
do
	finalfilename=$(echo $file | sed -e "s/\.tgs/\.png/g" ) 
	tgs-to-gif -f 30 $file 
	file="$file.gif"
	counter=1
	total=$(identify $file | wc -l )
	let "counter=$total/30"
	let "counter=$counter+1"
        if [ $counter -gt "7" ]
	then
		echo "You will be suffering here but I am continuing"
	fi
	#Running loop to find average frame delay , just for better working case
	let "total=$total-1"
	totalloop=0
	
	for i in {0..$total..1}
	do
		looptime=$(identify $file  | grep "\[$i\]" | sed "s/^.*\.//g")
		let "totalloop=$looptime+$totalloop"
	done

	let "total=$total+1"
        let "totalloop=$totalloop*$counter"
	let "totalloop=$totalloop/$total"
	rm output.gif
	log "Avg Frame Delay = $totalloop \n Total Frame = $total \n Counter Variable for delay = $counter \n File = $file "
	gifsicle -U $file  -d $totalloop  `seq -f "#%g" 0 $counter $total` -O9 --colors 256  -o output.gif
	
	file=output.gif
	convert -compress LZW   -coalesce $file  filepng%02d.png
	newf=$(echo $file | sed -e "s/\.gif/\.png/g" ) 
	
	apngasm $newf filepng*
	workplz $newf 
	mv tmp.png ./output/$finalfilename
	rm filepng*
done
rm *.tgs 
rm *.gif
rm *.png 
echo "Time to upload pack, conversion has been done!!!!"

if python3 bot.py 
then
	log "Pack uploaded"
  if [[ "$NOTIFY" == "" ]];
  then
	  notify-send "Pack converted here at $(pwd)"
  fi 
else
	echo "Pack wasn't uploaded run python bot.py in $(pwd)"
  if [[ "$NOTIFY" == "" ]];
  then
	   notify-send  "Pack wasn't uploaded run python bot.py in $(pwd)"
  fi
  cat pack >> not_uploaded
fi 
}

installdep
#Collect all tgs files in directory
rm pack 2>&1 > /dev/null
if [ -f token ] ;
then
	log "Token found continuing"
else
	post "Telegram Bot Token not found, \n Please use v1 if you have tgs , \n v2 always downloads tgs requires bot token \n 
 You can input token now or exit"
	echo $INPUU > token
	post "Enter author's name"
	echo $INPUU >> token
	post "Now open Signal Desktop ,\n Goto Menu -> Toggle Developers tools -> On there open Console \n Paste output of window.reduxStore.getState().items.uuid_id" 
	echo $INPUU >> token
	post  "You are almost there \n Paste output of window.reduxStore.getState().items.password"
	echo $INPUU >> token
fi
if [[ "$1" == "" ]] ;
then
    post "Please input link to pack to be converted eg https://t.me/addstickers/HalloUtya" 
    echo  $INPUU > pack
    if python3 download.py
		then
       maininstall
		 else
			 cat $INPUU >> not_uploaded 
		fi 
else
	   zmodload zsh/mapfile
     FNAME=$1
     FLINES=( "${(f)mapfile[$FNAME]}" )
     LIST="${mapfile[$FNAME]}" # Not required unless stuff uses it
     for iii in $FLINES
     do 
       echo "Installing" $iii
       echo  "$iii" > pack
       if python3 download.py
			 then 
					 maininstall
			 else
				 cat $iii >> not_uploaded
			 fi
			 echo -n "" > pack 
	   done 
fi

