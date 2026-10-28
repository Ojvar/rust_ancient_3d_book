#!/usr/bin/env sh
# Generates aged parchment pages and the book's material textures (ImageMagick only).
#   assets/pages/page-01.png … page-10.png   old-scripture pages
#   assets/textures/*.png                    leather, wood and page-edge maps
set -e
cd "$(dirname "$0")/.."
mkdir -p assets/pages assets/textures
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT

W=768; H=1076
FONT=/usr/share/fonts/gsfonts/C059-Roman.otf
CAP_FONT=/usr/share/fonts/gsfonts/C059-Bold.otf
INK="#2a1808"; RED="#8a1a10"; GOLD="#b8892c"

TEXT="In principio erat verbum et verbum erat apud deum et deus erat verbum hoc erat in principio apud deum omnia per ipsum facta sunt et sine ipso factum est nihil quod factum est in ipso vita erat et vita erat lux hominum et lux in tenebris lucet et tenebrae eam non conprehenderunt fuit homo missus a deo cui nomen erat iohannes hic venit in testimonium ut testimonium perhiberet de lumine ut omnes crederent per illum non erat ille lux sed ut testimonium perhiberet de lumine erat lux vera quae inluminat omnem hominem venientem in mundum in mundo erat et mundus per ipsum factus est et mundus eum non cognovit"
CAPS="A B C D E F G H I L"
ROMAN="I II III IV V VI VII VIII IX X"

# --- shared parchment base: fractal fibres, warm tint, burned vignette ---
magick -size ${W}x${H} -seed 7 plasma:fractal -colorspace gray -blur 0x4 -auto-level +level 72%,100% \
  \( -size ${W}x${H} xc: +noise Random -colorspace gray -blur 0x0.7 -auto-level +level 85%,100% \) \
  -compose multiply -composite \
  \( -size ${W}x${H} radial-gradient:white-black -level 0%,30% +level 30%,100% \) \
  -compose multiply -composite \
  +level-colors "#3a2008","#f2e0b0" "$tmp/parchment.png"

foxing() { # random brown spots and one larger stain
  seed=$1; awk -v s="$seed" -v w=$W -v h=$H 'BEGIN{srand(s);
    for(i=0;i<28;i++){x=rand()*w;y=rand()*h;r=2+rand()*7;
      printf "fill rgba(110,60,20,%.2f) circle %d,%d %d,%d ", 0.12+rand()*0.25,x,y,x+r,y}
    x=rand()*w;y=rand()*h;r=40+rand()*70;
    printf "fill rgba(120,75,25,0.10) circle %d,%d %d,%d", x,y,x+r,y}'
}

page() {
  i=$1; n=$(printf "%02d" "$i")
  cap=$(echo $CAPS | cut -d' ' -f"$i"); num=$(echo $ROMAN | cut -d' ' -f"$i")
  col="$tmp/col.png"
  magick -size 290x760 -background none -fill "$INK" -font "$FONT" -pointsize 21 \
    -interline-spacing 3 caption:"$TEXT" "$col"
  magick "$tmp/parchment.png" \
    -draw "$(foxing "$i")" \
    -stroke "$RED" -strokewidth 2 -fill none -draw "rectangle 52,52 716,1024" \
    -strokewidth 1 -draw "rectangle 60,60 708,1016" -stroke none \
    "$col" -geometry +72+220 -composite \
    "$col" -geometry +406+150 -composite \
    -fill "$GOLD" -draw "rectangle 72,100 202,210" \
    -fill "$RED" -draw "rectangle 78,106 196,204" \
    -font "$CAP_FONT" -pointsize 110 -fill "$GOLD" -gravity northwest -annotate +92+92 "$cap" \
    -gravity north -font "$FONT" -fill "$RED" -pointsize 30 -annotate +150+96 "CAPVT $num" \
    -fill "$GOLD" -draw "polygon 384,950 398,964 384,978 370,964" \
    -stroke "$RED" -strokewidth 2 -draw "line 250,964 362,964 line 406,964 518,964" -stroke none \
    -gravity south -fill "$INK" -pointsize 26 -annotate +0+36 "$i" \
    -attenuate 0.25 +noise Gaussian \
    "assets/pages/page-$n.png"
}

for i in 1 2 3 4 5 6 7 8 9 10; do page "$i"; done
cp "$tmp/parchment.png" assets/textures/endpaper.png

# --- leather: dark brown, wrinkled; height -> normal and roughness maps ---
T=512
magick -size ${T}x${T} -seed 3 plasma:fractal -colorspace gray -blur 0x1.2 \
  \( -size ${T}x${T} xc: +noise Random -colorspace gray -blur 0x0.8 \) \
  -compose overlay -composite -auto-level "$tmp/leather-h.png"
magick "$tmp/leather-h.png" -fill "#3a1d0e" -tint 100 -level 15%,85% \
  +level-colors "#1e0c05","#5a2e17" assets/textures/leather.png
magick "$tmp/leather-h.png" -blur 0x1.5 -define convolve:scale='40%!' -bias 50% \
  \( -clone 0 -morphology Convolve Sobel:0 \) \
  \( -clone 0 -morphology Convolve Sobel:90 \) \
  \( -clone 0 -fill white -colorize 100 \) -delete 0 \
  -combine -set colorspace RGB assets/textures/leather-normal.png
# glTF layout: green = roughness, blue = metallic (kept 0)
magick "$tmp/leather-h.png" -level 0%,100% +level 65%,95% \
  \( -clone 0 \) \( -clone 0 -fill black -colorize 100 \) \
  \( -clone 0 \) -delete 0 -combine -set colorspace RGB assets/textures/leather-roughness.png

# --- wood: stretched noise grain ---
magick -size 64x${T} xc: -seed 5 +noise Random -colorspace gray -blur 0x3 \
  -resize ${T}x${T}! -motion-blur 0x12+90 -auto-level \
  +level-colors "#1a0d06","#4a2814" assets/textures/wood.png

# --- page edges: fine horizontal lines of stacked paper ---
magick -size 8x256 xc: -seed 9 +noise Random -colorspace gray -resize 256x256! \
  -auto-level +level-colors "#8a6a3a","#e2c995" assets/textures/page-edge.png

# 8-bit PNGs: Bevy loads 16-bit PNGs as linear data, which washes colours out.
magick mogrify -depth 8 assets/pages/*.png assets/textures/*.png

echo "Wrote 10 pages and textures"
