for file in textures/**; do
	echo $file
	./crop_image_to_square.sh $file $file
done
