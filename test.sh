cd ~/Repos
git clone https://github.com/catppuccin/grub.git && cd grub
sudo cp -r src/* /usr/share/grub/themes/
GRUB_THEME="/usr/share/grub/themes/catppuccin-mocha-grub-theme/theme.txt"



# Define the new theme path

# Use sed to find and replace the line starting with GRUB_THEME
sudo sed -i "s|^GRUB_THEME=\".*\"|GRUB_THEME=\"$GRUB_THEME\"|" /etc/default/grub


if [ $? -eq 0 ]; then
    echo "GRUB_THEME updated successfully."
else
    echo "Failed to update GRUB_THEME."
fi


# Update GRUB configuration
sudo grub-mkconfig -o /boot/grub/grub.cfg