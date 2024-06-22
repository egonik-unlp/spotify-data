git clone https://github.com/egonik-unlp/my-spotify-data.git
cp -r my-spotify-data/extended_streaming_history .
rm -rf my-spotify-data
filepath=full_str_redux.json
rm -f $filepath
for file in ./extended_streaming_history/Streaming_History_Audio*;
	do tail -n  +1 $file | head -n -1 >> $filepath 
	echo "," >> $filepath
done;