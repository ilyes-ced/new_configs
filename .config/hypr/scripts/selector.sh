#!/bin/bash
# source: https://github.com/Igneel0601/wall-engine

# Static wallpaper directory
STATIC_WALLPAPER_DIR="$HOME/Pictures/wallpapers" 
LIVE_WALLPAPER_DIR="$HOME/.config/hypr/wallpapers/live-walls"

# Thumbnail cache directory
THUMB_DIR="$HOME/.cache/wallpaper_thumbs"
mkdir -p "$THUMB_DIR"

# Rofi theme
ROFI_THEME="$HOME/.config/rofi/selector.rasi"

# Check for required programs
for cmd in swww rofi; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "$cmd is not installed. Please install it."
        exit 1
    fi
done

# Check for ImageMagick
if ! command -v convert &>/dev/null && ! command -v magick &>/dev/null; then
    echo "ImageMagick is required for thumbnail generation."
    exit 1
fi

convert_cmd=$(command -v convert || command -v magick)

generate_thumbnails() {
    echo "Generating thumbnails..."

    for dir in "$STATIC_WALLPAPER_DIR" "$LIVE_WALLPAPER_DIR"; do
        for ext in jpg jpeg png webp bmp gif mp4 mkv; do
            shopt -s nullglob
            for img in "$dir"/*."$ext"; do
                [ -f "$img" ] || continue
                filename=$(basename "$img")
                name="${filename%.*}"
                thumb_path="$THUMB_DIR/${name}_thumb.png"

                if [ ! -f "$thumb_path" ] || [ "$img" -nt "$thumb_path" ]; then
                    if [[ "$img" =~ \.(mp4|mkv)$ ]]; then
                        # ffmpeg -i "$img" -vf "scale=500:500" -frames:v 1 "$thumb_path" -y 2>/dev/null
                        ffmpeg -i "$img" -vf "scale=500:500" -vframes 1 "$thumb_path" -y 2>/dev/null
                        # ffmpeg -i "$img" -frames:v 1 "$thumb_path" -y 2>/dev/null
                        # "$convert_cmd" "$thumb_path" -strip -thumbnail 500x500^ -gravity center -extent 500x500 "$thumb_path" 2>/dev/null

                    else
                        "$convert_cmd" "$img[0]" -strip -thumbnail 500x500^ -gravity center -extent 500x500 "$thumb_path" 2>/dev/null
                    fi
                fi
            done
            shopt -u nullglob
        done
    done
}

create_rofi_entries() {
    mapping_file="/tmp/wallpaper_mapping_$$"
    > "$mapping_file"

    for dir_type in static live; do
        dir_var="${dir_type^^}_WALLPAPER_DIR"
        dir="${!dir_var}"

        for ext in jpg jpeg png webp bmp gif mp4 mkv; do
            for img in "$dir"/*."$ext"; do
                [ -f "$img" ] || continue
                filename=$(basename "$img")
                name="${filename%.*}"
                label="$name"
                [ "$dir_type" = "live" ] && label="[Live] $name"
                thumb="$THUMB_DIR/${name}_thumb.png"

                echo -e "$label|$img|$dir_type" >> "$mapping_file"

                if [ -f "$thumb" ]; then
                    echo -e "$label\x00icon\x1f$thumb"
                else
                    echo "$label"
                fi
            done
        done
    done
}

generate_thumbnails

if [ -f "$ROFI_THEME" ]; then
    selection=$(create_rofi_entries | rofi -dmenu -i \
        -p "󰸉 Select Wallpaper :" \
        -show-icons \
        -markup-rows \
        -theme "$ROFI_THEME")
else
    selection=$(create_rofi_entries | rofi -dmenu -i \
        -p "󰸉 Select Wallpaper" \
        -show-icons \
        -markup-rows \
        -theme-str 'listview { columns: 3; lines: 3; }' \
        -theme-str 'element { orientation: vertical; }' \
        -theme-str 'element-text { horizontal-align: 0.5; }' \
        -theme-str 'element-icon { size: 5em; }')
fi

[ -z "$selection" ] && exit 0

mapping_file="/tmp/wallpaper_mapping_$$"
read -r selected_line < <(grep -F "$selection" "$mapping_file")
IFS='|' read -r _ selected_path type <<< "$selected_line"

rm -f "$mapping_file"

if [ -f "$selected_path" ]; then
    cd ~/new_configs/scripts/src
    cargo run --bin change_theme -- --type=pywal --backend=colorthief --wallpaper="$selected_path" && ../../copy_configs.sh
    cd
else
    echo "Error: File not found - $selected_path"
    exit 1
fi