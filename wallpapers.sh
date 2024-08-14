#!/bin/bash

url_file="wallpapers.txt"

# Check if the file exists
if [ ! -f "$url_file" ]; then
    echo "File not found: $url_file"
    exit 1
fi

# Set the download directory
download_dir=~/Pictures/wallpapers

# Create the download directory if it doesn't exist
mkdir -p "$download_dir"

# Loop through each URL in the file and download it
while IFS= read -r url; do
    # Extract the filename from the URL
    filename=$(basename "$url")
    
    # Download the image using curl and save it in the download directory
    echo "Downloading $url..."
    curl -o "$download_dir/$filename" "$url"
    
    # Check if the download was successful
    if [ $? -eq 0 ]; then
        echo "Downloaded: $filename to $download_dir"
    else
        echo "Failed to download: $filename"
    fi
done < "$url_file"
