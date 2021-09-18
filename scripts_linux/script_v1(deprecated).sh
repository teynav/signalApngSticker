#!/bin/zsh
log()
{ 
	echo $1
}
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
		        log "removing $file at index $i" 
			rm $file 
			break 
	           fi
              done
	if [ $got -eq "0" ]
	then
		let "count=$count+1"
		mv $file "filepng"${ref[$count]}".png"
		log "${ref[$count]}"
		
	fi
        let "val=$val+1"
        done
}
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
mkdir output
for file in ./*.tgs 
do
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
	#if [ $total -gt 180 ]
	#then
	#	log "EWWWWWWWWWWW $file shove it plzz"
	#	log "So many frames I will shut down , Sowwie Plzzz"
	#	exit 1
	#elif [ $total -gt 150 ]
	#then 
	#	counter=6
	#elif [ $total -gt 120 ]
	#then
	#       counter=5
	#elif  [ $total -gt 90 ]
	#then
	#	counter=4
	#elif  [ $total -gt 60 ] 
	#then
	#	counter=3
	#elif  [ $total -gt 30 ]
	#then
	#	counter=2
	#else
	#	counter=1
	#fi
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
	mv tmp.png ./output/$(uuidgen).png
	rm filepng*
done
