#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: rechatdownload.sh https://api.twitch.tv/v5/videos/.../comments?client_id=..."
    echo "This should place files like 1.json, 2.json and so on to current directory"
    echo "Use rechat2srt then to convert series of JSONs and a basetime to SRT subtitles"
fi

URI="$1"
N=1
CURSOR=

set -e

while true; do
    if [ -e "$N.json" ]; then
        echo "$N.json already exists"
    else
        curl "$URI""$CURSOR" -o "$N.json"
    fi
    ID=$(cat $N.json  | jq -r '._next')
    if [ "$ID" == "null" ]; then
        echo "That's seems to be the end of it"
        break
    fi
    CURSOR='&cursor='"$ID"
    : $((N+=1))
done
