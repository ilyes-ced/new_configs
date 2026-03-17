GTK and ICON themes source: 

[GTK theme](https://github.com/vinceliuice/Colloid-gtk-theme)

[Icon theme](https://github.com/vinceliuice/Colloid-icon-theme)






clone this repo in the Home HOME directoty ~/

```
cd new_configs/scripts/src

cargo run --bin change_theme -- --type=pywal --backend=colorthief --wallpaper=random && ../../copy_configs.sh && neofetch
```