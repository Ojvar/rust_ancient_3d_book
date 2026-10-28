#!/usr/bin/env sh
# Generates 10 simple sample page images into assets/pages (uses ImageMagick).
set -e
cd "$(dirname "$0")/.."
mkdir -p assets/pages
colors="#e76f51 #f4a261 #e9c46a #2a9d8f #264653 #8ab17d #6d597a #b56576 #457b9d #1d3557"
i=1
for c in $colors; do
  n=$(printf "%02d" "$i")
  magick -size 512x716 "xc:$c" \
    -fill "#ffffff33" -draw "circle 256,330 256,150" \
    -fill white -gravity center -pointsize 220 -annotate +0-20 "$i" \
    -pointsize 36 -annotate +0+290 "Page $n" \
    -bordercolor "#f5f0e6" -border 24 \
    "assets/pages/page-$n.png"
  i=$((i + 1))
done
echo "Wrote assets/pages/page-01.png ... page-10.png"
