convert $1  -trim -background none \
   -set page "%[fx:max(w,h)]x%[fx:max(w,h)]+%[fx:(max(w,h)-w)/2]+%[fx:(max(w,h)-h)/2]" \
   -coalesce $2
