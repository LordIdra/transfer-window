#!/bin/bash

rm view/resources/textures/cache/**

cd view/resources/textures/system || exit
for file in **; do
   gmic "$file" -gaussian-blur 0x8 -o "../cache/$file"
   convert "$file" "../cache/$file" -compose screen -composite "../cache/$file"
done

# cd ../ship || exit
# for file in **; do
#    gmic "$file" -blur 0x2 -o "../cache/$file"
#    # convert "$file" "../cache/$file" -compose screen -composite "../cache/$file"
#    # convert "$file" "../cache/$file" -compose screen -composite "../cache/$file"
# done