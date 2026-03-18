#!/bin/bash

cp ~/new_configs/scripts/active/hyprland.conf ~/.config/hypr/theme.conf
cp ~/new_configs/scripts/active/dunstrc ~/.config/dunst/dunstrc
cp ~/new_configs/scripts/active/waybar.css ~/.config/waybar/current-theme.css
cp ~/new_configs/scripts/active/wlogout.css ~/.config/wlogout/style.css
cp ~/new_configs/scripts/active/zellij.kdl ~/.config/zellij/config.kdl
cp ~/new_configs/scripts/active/rofi.rasi ~/.config/rofi/colors.rasi
cp ~/new_configs/scripts/active/starship.toml ~/.config/starship.toml
cp ~/new_configs/scripts/active/alacritty.toml ~/.config/alacritty/colors.toml
cp ~/new_configs/scripts/active/gtk.scss ~/new_configs/Colloid-gtk-theme/src/sass/_color-palette-custom.scss
cp ~/new_configs/scripts/active/install.sh ~/new_configs/Colloid-icon-theme/install.sh
cp ~/new_configs/scripts/active/config.py ~/.config/qutebrowser/


swww img ~/new_configs/scripts/active/wallpaper 


cd ~/new_configs/Colloid-gtk-theme
echo "changing GTK theme:"
./install.sh --tweaks custom --tweaks black -s compact -c dark
cd ../..



cd ~/new_configs/Colloid-icon-theme
echo "changing Icons theme:"
./install.sh -s custom -b -t all
cd ../..


