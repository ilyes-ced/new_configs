#!/bin/bash


#first promt : custom or pywal

#if custom
#    promt theme name
#    promt wallpaper 
#if pywal
#    promt backend 
#    promt wallpaper 



#!/bin/bash

# Function to display usage
usage() {
    echo "Usage:"
    echo "  $0"
    exit 1
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


# Function to check if a file exists
check_file_exists() {
    local file_path=$1
    echo $file_path
    if [[ -f "$file_path" ]]; then
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
            if check_file_exists "$HOME/new_configs/themes/$theme_name.json"; then
                break
            else
                echo "Theme doesnt exist please choose between <catppucin|dracula|nord|tokyonight|rose pine>"
            fi
        done
        # If custom is chosen, prompt for theme name and wallpaper
        while true; do
            read -p "Enter wallpaper path: " wallpaper_path

            # Validate the input
            if check_file_exists "$HOME/Pictures/wallpapers/$wallpaper_path"; then
                break
            else
                echo "Wallpaper doesnt exist"
            fi
        done
        # here we add the moving part
        # either using the rust script or by making a new one
        echo "You have chosen a custom theme."
        echo "Theme Name: $theme_name"
        echo "Wallpaper Path: $wallpaper_path"
        ;;
    pywal)
        # If pywal is chosen, prompt for backend and wallpaper
        read -p "Enter backend <wal|colorthief>: " backend
        read -p "Enter wallpaper path: " wallpaper_path
        
        echo "You have chosen pywal."
        echo "Backend: $backend"
        echo "Wallpaper Path: $wallpaper_path"
        ;;
esac