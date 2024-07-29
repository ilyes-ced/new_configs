#!/bin/bash


#first promt : custom or pywal

#if custom
#    promt theme name
#    promt wallpaper 
#if pywal
#    promt backend 
#    promt wallpaper 



# run the theme templating and changing
change_theme() {
    cd ~/new_configs/scripts
    cargo run --bin main -- --theme_name=$(basename $theme_path) --wallpaper=$(basename $wallpaper_path) && i3-msg restart && neofetch
    # use the rust script 
    # restart i3
    # run neofetch
}


# Function to validate input
validate_input() {
    local choice=$1
    
    case $choice in
        custom|pywal)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}



theme_path=""
wallpaper_path=""
check_file_exists() {
    if [[ "$2" == "theme" ]]; then
        if [[ "$1" == "random" ]]; then
            selected=$(find "$HOME/new_configs/themes/" -type f | shuf -n 1)
            theme_path=$selected
            echo $selected
        else
            theme_path=$HOME/new_configs/themes/$1.json
        fi
    elif [[ "$2" == "wallpaper" ]]; then
        if [[ "$1" == "random" ]]; then
            selected=$(find "$HOME/Pictures/wallpapers/" -type f | shuf -n 1)
            wallpaper_path=$selected
            echo $selected
        else
            wallpaper_path=$HOME/Pictures/wallpapers/$1
        fi
    else 
        echo "needs to be theme or wallaperp"
    fi





    

    
    if [[ -f "$theme_path" ]]; then
        return 0  # File exists
    else
        return 1  # File does not exist
    fi
    if [[ -f "$wallpaper_path" ]]; then
        return 0  # File exists
    else
        return 1  # File does not exist
    fi
}



# Loop until a valid choice is made
while true; do
    read -p "Choose an option (custom/pywal): " choice
    
    # Validate the input
    if validate_input "$choice"; then
        break
    else
        echo "Invalid option. Please choose either 'custom' or 'pywal'."
    fi
done

case $choice in
    custom)
        # If custom is chosen, prompt for theme name and wallpaper
        while true; do
            read -p "Enter theme name: " theme_name

            # Validate the input
            if check_file_exists "$theme_name" "theme"; then
                break
            else
                echo "Theme doesnt exist please choose between <catppucin|dracula|nord|tokyonight|rose pine>"
            fi
        done
        # If custom is chosen, prompt for theme name and wallpaper
        while true; do
            read -p "Enter wallpaper path: " wallpaper_name

            # Validate the input
            if check_file_exists "$wallpaper_name" "wallpaper"; then
                break
            else
                echo "Wallpaper doesnt exist"
            fi
        done
        # here we add the moving part
        # either using the rust script or by making a new one
        echo "finished the selection. starting the changing"
        echo $(basename $theme_path)
        echo $(basename $wallpaper_path)
        change_theme
        ;;
    pywal)
        # If pywal is chosen, prompt for backend and wallpaper
        read -p "Enter backend <wal|colorthief>: " backend
        read -p "Enter wallpaper path: " wallpaper_name
        
        echo "You have chosen pywal."
        echo "Backend: $backend"
        echo "Wallpaper Path: $wallpaper_name"
        ;;
esac



