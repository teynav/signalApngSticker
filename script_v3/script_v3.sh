#!/bin/bash 
#Implemented properly in v4
log()
{ 
	echo $1
}

#Gets input here from post function
INPUU=""
#Tells if we are using GUI 
NOTIFY="NO"
IN_BACKUP="0"
TOTALF="0"
FILE_I="$1"
let "COUNTER=0"
 if [[ "$(tty)" == "not a tty" ]] ;
 then 
      NOTIFY=""
 fi 

info () {
 if [[ "$NOTIFY" == "" ]];
 then 
	 if [[ "$2" == "" ]];
	 then
		 notify-send "$1"
	 else
		 zenity --info --text="$1" --width="400"
	 fi
 else
	 echo "$1"
 fi
}

post () {
#	notify-send "In post with $1 and $(tty)"
if [[ "$NOTIFY" == "" ]] ;
then
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
		printf "$1\n"
		read INPUU
fi
}
#v3 related changed down here !

# For what is package manager 
depi () {
	if command -v apt > /dev/null 
	then 
		log "Installing $1 for ubuntu"
		if sudo apt install $1  ; 
		then
			log "installed" 
		else
			exit 
		fi 
	elif command -v pacman > /dev/null 
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

	if command -v $2 > /dev/null 
	then 
		log " "
	else
		log "Exiting because unable to install dependencies"
	fi 
}
aur ()
{
	if command -v yay > /dev/null 
	then
		yay -S $1  
	elif command -v pikaur > /dev/null
	then
		pikaur  -S $1  
	elif command -v paru > /dev/null 
	then 
		paru -S $1 
	else 
		echo "Input name of your AUR-Helper Manually"
		read aurh
		if $aurh -S $1 > /dev/null 
		then
			log "Installed $1"
		else 
			log "Can't install this "
			exit 1 
		fi 
	fi
	if command -v $2 > /dev/null 
	then
		log "Installed $1"
	else
		log "Couldn't install $1"
		exit 1
	fi 
}
pyins () {

if command -v pip3 > /dev/null 
 then 
   if sudo pip3 install $4 --no-input 
	 then 
     echo -n ""
	 else
		 log "Check internet Connection !! "
		 exit 1
	 fi 
 else
   if [[ "$1" == "" ]];
   then
	   log "Please install python-pip on your system"
   exit 1
   fi
	 if [[ "$2" == "aa" ]];
	 then
		 log "Couldn't install python-pip on your system"
		 exit 1
	 fi
	if  sudo $1 $2 $3 
	then
		pyins $1 aa $3 $4 
	else
    exit 1 
	fi 
fi
}
pydep () {
	log "Checking python dependencies"
	local a=""
	local b=""
	local c=""
	if command -v apt  > /dev/null
	then
		a="apt"
		b="install"
		c="python3-pip"
	elif command -v pacman > /dev/null 
	then
		a="pacman"
		b="-Syuu"
		c="python-pip"
	fi 
	if python -m "telegram" > /dev/null 2>&1 
	then 
		echo -n ""
	else
		log "Install python telegram bot"
		pyins $a $b $c python-telegram-bot 
   fi 
	 if python -c "import signalstickers_client" > /dev/null 2>&1 
	 then
		 echo -n ""
	 else
		 log "Install SignalStickers Client"
		 pyins $a $b $c signalstickers-client
	 fi 
}

dobackup()
{
	if [[ ! -d .backup ]];
	then
		mkdir .backup 
	fi 
	if [[ ! -f .back ]];
	then
		touch .back
	fi 
	for i in {0..100..1}
	do 
		if [[ -d .backup/$i ]];
		then
			continue
		fi 
    mkdir .backup/$i 
	  echo $i >> .back 
		cp -rf result .backup/$i/ 
		cp emoji .backup/$i/
		cp pack .backup/$i/ 
		log "Couldn't Upload hence backup created in folder .output/$i , rerun script to do Upload"
		break
	done
}
installbak() {
a=$(cat .back)
SAVEIFS=$IFS   # Save current IFS
IFS=$'\n'      # Change IFS to new line
a=($a)
ee=$$
for i in "${a[@]}" 
   do
		  rm -rf result
      cp -f .backup/$i/pack pack
		  cp -rf .backup/$i/result result
		  cp -r .backup/$i/emoji emoji 

		  if python3 bot.py 2> /dev/null
		  then
			  log "Pack $i $(cat pack) uploaded"
			  sed "/^$i$/d" .back > .lss
				mv .lss .back 
      else
			  info "Please check network connection!!!" 1
			exit 1
		  fi
		  rm -rf .backup/$i
			rm -rf result
		done 
echo "-------"
info "All backups have been uploaded !!! Continuing?" 1 
echo "-------"
echo -n "" > pack 
echo -n "" > emoji
rm -rf result 
}


# Starts checking dependencies Here 
installdep () {
	if  command -v apngasm  > /dev/null 
	then
		echo -n ""
	else
		log "Trying to install apngasm"
		if command -v apt > /dev/null 
		then
			sudo apt install apngasm 
		elif command -v pacman > /dev/null
		then
			aur apng-utils apngasm
		fi 
	fi 
  
	if  command -v apngdis  > /dev/null 
	then
		echo -n ""
	else
		log "Trying to install apngdis"
		if command -v apt > /dev/null 
		then
			sudo apt install apngdis
		elif command -v pacman > /dev/null
		then
			log "Might tell to uninstall the package apngasm,\n Which would not be a problem as it would be installed in apng-utils"
			aur apng-utils apngdis 
		fi 
	fi 


	if  command -v optipng  > /dev/null 
	then
	      echo -n ""
	else
		log "Trying to install optipng"
		depi optipng optipng  
	fi 
   
  if  command -v gzip  > /dev/null 
	then
	      echo -n ""
	else
		log "Trying to install gzip"
		depi gzip gzip
	fi 

  if  command -v pngquant  > /dev/null 
	then
		echo -n ""
	else
		log "pngquant is not installed, Trying to install it"
		depi pngquant pngquant 
	fi 
  
  if  command -v pngnq-s9  > /dev/null 
	then
		echo -n ""
	else
		if command -v apt > /dev/null 
		then
			if command -v pngnq > /dev/null 
			then 
				echo -n ""
			else
	       	log "Trying to install pngnq"
			    sudo apt install pngnq
			fi 
		elif command -v pacman > /dev/null
		then
		log "Trying to install pngnq-s9"
			aur pngnq-s9 pngnq-s9 
		fi 
	fi 

	if  command -v convert  > /dev/null 
	then
		echo -n ""
	#	log "Imagemagick is installed !!!!!!!!"
	else
		log "Imagemagick is not installed, Trying to install it"
		depi imagemagick convert 
	fi 
		
	if  command -v tgs2png  > /dev/null 
	then
		echo -n "" 
	else 
		if command -v pacman > /dev/null 
		then 
			aur tgs2png-git tgs2png 
		else 
         echo "Please install tgs2png manually"
				 exit 1 
		fi 
	fi
	pydep 
}

# Just ports all tgs to png 
porter () 
{
if [[ ! -d ./result ]]
then
   mkdir result 
fi

rm -rf testdir
mkdir testdir 
for file in ./*.tgs 
do
  jobbb=`jobs | wc -l`
	while [[ $jobbb -ge "3" ]]
  do
		#echo "Total jobs = $(jobs | wc -l )"
    sleep 5
		jobbb=`jobs | wc -l`
  done 
	batch=$(echo $file | sed "s/.tgs//g")
	./core-hybrid $batch $file &
done 
wait 

}
#Just downloads the pack ! 

getpack() {
      if python3 download.py 2> /dev/null 
	    then
         echo "Downloaded it all "
		  else
				info "Can't download pack $(cat pack)"
			   cat pack >> not_uploaded
				 exit 1
		  fi
}

#Just uploads the pack 
uploader() {
if python3 bot.py 2> /dev/null 
then
	info  "Pack uploaded $(cat pack)"
else
	info "Pack wasn't uploaded Doing backup !! "
  dobackup
  #cat pack >> not_uploaded
fi
}
# Step 1 . Check all dependencies
installdep


# Step 2 . Check if we have all the tokens to manage manual work !
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


# Step 3 . See if there is backup 

if [[  -s .back ]];
then
  if [[ "$NOTIFY" == "" ]] ;
  then
       if zenity --question --text "Backup file found, Do you want to upload those stickers which are left out?" --width=400 --title="Sticker Pack Creator <3" 
			 then
            installbak 
       fi
	else
		echo  "Backup file found, Do you want to upload those stickers which are left out? (N/y)"
    read xxx 
		if [[ "$xxx" == "y" ]];
		then 
		   installbak
		elif [[ "$xxx" == "Y" ]];
		then
			installbak
		else
			log "Skipped backup"
		fi
	fi
fi

#Step 4 . Take an Input (GUI part unimplemented )
rm -rf pack 
takein() {
	if [[ "$NOTIFY" == "" ]]
	then
		INPUU=$(zenity --text="Enter Link Here " --title="Sticker Pack Creator <3" --entry  --ok-label="Convert this" --extra-button="Choose file")
		if [[ "$INPUU" == "Choose file" ]]
		then
		   INPUU=$(zenity --file-selection )
			 FILE_I=$INPUU
		else
       echo $INPUU > pack 
		fi 
		if [[ "$INPUU" == "" ]]
		then
			notify-send "Cancelling conversion"
			exit 1
		fi 
	else
		###
		if [[ "$1" == "" ]]
		then 
		  post "Please input link to pack to be converted eg https://t.me/addstickers/HalloUtya" 
      echo  $INPUU > pack
		fi
	fi 
}

takein $1 


# Check for other flags

if [[ $1 == "--convert" ]]
then 
	porter
	exit
elif [[ $1 == "--help" ]] 
then 
	log "v3.0 Start script with \n\n ./script_v3.sh <filename> \n ./script_v3.sh \n ./script_v3.sh --convert ( Just converts the given tgs to apng)"
	exit 
fi 


if [[ "$FILE_I" == "" ]] ;
then
		 getpack 
		 porter 
		 uploader
		 rm *.tgs
else
     TOT=$(cat $FILE_I | wc -l )
		 conv=1 
	   aaaa=$(cat $FILE_I)
     SAVEIFS=$IFS   # Save current IFS
     IFS=$'\n'      # Change IFS to new line
     aaaa=($aaaa)
		 IFS=$SAVEIFS
		 # split to array $names
     for iii in "${aaaa[@]}"
     do 
     echo "# Installing" $iii
     echo  "$iii" > pack
		    getpack 
        porter
				uploader 
				rm *.tgs 
		 let "conv=$conv+1"
	   done
fi
