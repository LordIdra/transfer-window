#!/bin/bash

cd view/resources/textures/system || exit

for file in **; do
   gmic "$file" -blur 0x8 -o "blur_$file"
   convert "$file" "blur_$file" -compose screen -composite "$file"
done