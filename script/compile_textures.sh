#!/bin/bash

root=$(pwd)

export_image() {
    # rm "$output"
    drawio --export --scale 0.5 --transparent --output "$root/view/resources/final_textures/${1%.*}.png" "$1" 2>/dev/null
}

bloom_system() {
    image="$root/view/resources/final_textures/${1%.*}.png"
    blur_image="$root/view/resources/final_textures/blur_${1%.*}.png"
    convert "$image" -channel RGBA -gaussian-blur 0x4 "$blur_image"
    convert "$image" "$blur_image" -compose screen -composite "$image"
    convert "$image" "$blur_image" -compose screen -composite "$image"
    rm "$blur_image"
}

bloom_ship() {
    image="$root/view/resources/final_textures/${1%.*}.png"
    blur_image="$root/view/resources/final_textures/blur_${1%.*}.png"
    convert "$image" -channel RGBA -gaussian-blur 0x8 "$blur_image"
    convert "$image" "$blur_image" -compose screen -composite "$image"
    convert "$image" -channel RGBA -gaussian-blur 0x8 "$blur_image"
    convert "$image" "$blur_image" -compose screen -composite "$image"
    rm "$blur_image"
}

bloom_menu() {
    image="$root/view/resources/final_textures/${1%.*}.png"
    blur_image="$root/view/resources/final_textures/blur_${1%.*}.png"
    convert "$image" -channel RGBA -gaussian-blur 0x2 "$blur_image"
    convert "$image" "$blur_image" -compose screen -composite "$image"
    rm "$blur_image"
}

export_and_bloom_system() {
    export_image "$1"
    bloom_system "$1"
}

export_and_bloom_ship() {
    export_image "$1"
    bloom_ship "$1"
}

export_and_bloom_menu() {
    export_image "$1"
    bloom_menu "$1"
}

# cd "$root/view/resources/textures/system" || exit
# N=8 # https://unix.stackexchange.com/questions/103920/parallelize-a-bash-for-loop
# (
#     for file in *.drawio; do
#         ((i=i%N)); ((i++==0)) && wait
#         [ -f "$file" ] || break
#         export_and_bloom_system "$file" &
#     done
# )
# wait

# cd "$root/view/resources/textures/ship" || exit
# for file in *.drawio; do
#     [ -f "$file" ] || break
#     export_and_bloom_ship "$file" &
# done
# wait

cd "$root/view/resources/textures/menu" || exit
for file in *.drawio; do
    [ -f "$file" ] || break
    export_and_bloom_menu "$file" &
done
wait

cd "$root/view/resources/textures/character" || exit
for file in *.drawio; do
    [ -f "$file" ] || break
    export_and_bloom_menu "$file" &
done
wait

cd "$root/view/resources/textures/icon" || exit
cp ./*.png "$root/view/resources/final_textures"

cd "$root/view/resources/textures/celestial_object" || exit
cp ./*.png "$root/view/resources/final_textures"