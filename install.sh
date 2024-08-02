#!/bin/bash

############################################
############################################
############################################
# update system
############################################
############################################
############################################
sudo pacman -Syu --noconfirm
sudo pacman -S archlinux-keyring --noconfirm
# needed to isntall vecodium at the time of writing this
sudo pacman -S debugedit --noconfirm
mkdir ~/Repos ~/Installs ~/Projects



############################################
############################################
############################################
# installing pacman packages
############################################
############################################
############################################
# Path to the package list file
pacman_packages="pacman.txt"

echo "Installing pacman packages from the list..."
sudo pacman -S --noconfirm i3-wm i3blocks i3status mpv feh
echo "All pacman packages installed successfully!"






############################################
############################################
############################################
# installing yay packages
############################################
############################################
############################################
# Path to the package list file
yay_packages="yay.txt"

echo "Installing yay packages from the list..."
yay -S --noconfirm vscodium brave floorp-bin greenclip
echo "All yay packages installed successfully!"








############################################
############################################
############################################
# installing firefox secure script
############################################
############################################
############################################
cd ~/Repos
git clone https://github.com/simeononsecurity/FireFox-Privacy-Script --depth 1
cd FireFox-Privacy-Script
sudo chmod +x ./sos-firefoxprivacy.sh
sudo bash ./sos-firefoxprivacy.sh





############################################
############################################
############################################
# installing vscode extention + setting the config file
############################################
############################################
############################################
extensions=(
    "GitHub.github-vscode-theme"
    "naumovs.color-highlight"
    "PKief.material-icon-theme"
    "rust-lang.rust-analyzer"
    "tamasfe.even-better-toml"
    "serayuzgur.crates"
    "ms-vscode.vscode-typescript-next"
    "aaron-bond.better-comments"
    "pranaygp.vscode-css-peek"
    "streetsidesoftware.code-spell-checker"
    "yzhang.markdown-all-in-one"
)

# Loop through the array and install each extension
for extension in "${extensions[@]}"; do
    echo "Installing $extension..."
    codium --install-extension "$extension"
done

echo "All extensions have been installed."

















############################################
############################################
############################################
# misc
############################################
############################################
############################################
# nvchad
# note: also add ur configs for nvim later
git clone https://github.com/NvChad/starter ~/.config/nvim



# installing rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
if command -v rustc &> /dev/null; then
    echo "Rust installed successfully!"
    rustc --version
else
    echo "Rust installation failed."
fi


# installing eww
#cd ~/Repos
#git clone https://github.com/elkowar/eww  --depth 1
#cd eww
#cargo build --release --no-default-features --features x11
#cp target/release/eww ~/Installs
#cd ~/Installs
#chmod +x ./eww
#mkdir ~/.config/eww
#./eww daemon
#./eww open <window_name>



# installing zsh extentions x2
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
sed -i '/^plugins=(/ s/plugins=(\([^)]*\))/plugins=(\1 zsh-syntax-highlighting zsh-autosuggestions)/' .zshrc
# pokemon scripts for zsh
cd ~/Repos
git clone https://gitlab.com/phoneybadger/pokemon-colorscripts.git  --depth 1
cd pokemon-colorscripts
sudo ./install.sh
echo "pokemon-colorscripts -r" >> ~/.zshrc



# installing atuin
curl --proto '=https' --tlsv1.2 -LsSf https://setup.atuin.sh | sh
echo 'eval "$(atuin init zsh)"' >> ~/.zshrc




# installing zellij
mkdir ~/.config/zellij
cargo install --locked zellij
echo 'eval "$(zellij setup --generate-auto-start zsh)"' >> ~/.zshrc



# git username and email
git config --global user.name "ilyes-ced"
git config --global user.email "random_dude_233@proton.me"












############################################
############################################
############################################
# not needed i3 is enough for now
############################################
############################################
############################################

# add polybar
# still not sure if i want to use it
# sudo pacman -S polybar
# mkdir ~/.config/polybar


# add dwm + patches later

# add dwm desktop
# sudo cp dwm.desktop /usr/share/xsessions/
# sudo make install clean ~/new_configs/suckless/dwm/
# sudo make install clean ~/new_configs/suckless/dmenu/
# sudo make install clean ~/new_configs/suckless/st/


# maybe add eww for dwm or hyprland
# set desktop keyboard map to fr

# add some tweakks to hyprdots

############################################
############################################
############################################









# add i3 configs
cp -r i3/* ~/.config



# add gtk themes



# add firefox extentions


# add option to save password and use it later
# add option to select a WM or multiple









# add blackarch repo + maybe most useful tools
# causes too much problems do it later manually
## cd ~/Repos
## curl -O https://blackarch.org/strap.sh
## echo 26849980b35a42e6e192c6d9ed8c46f0d6d06047 strap.sh | sha1sum -c
## chmod +x strap.sh
## sudo ./strap.sh
## # NOTE: could fail
## sudo pacman -Syu --noconfirm
##





# sddm theme
cd ~/Repos
curl -L https://github.com/catppuccin/sddm/releases/download/v1.0.0/catppuccin-mocha.zip > mocha.zip
sudo unzip mocha.zip -d /usr/share/sddm/themes/

# turn numlock on
FILE="/etc/sddm.conf"
sudo sed -i 's/Numlock=off/Numlock=on/' "$FILE"
if [ $? -eq 0 ]; then
    echo "Successfully changed Numlock=off to Numlock=on in $FILE"
else
    echo "Failed to modify the file."
fi

sudo sed -i '/\[Theme\]/a Current=catppuccin-mocha' "$FILE"
if [ $? -eq 0 ]; then
    echo "Successfully added 'Current=catppuccin-mocha' after '[Theme]' in $FILE"
else
    echo "Failed to modify the file."
fi




# /////////////////////////////////////////
# /////////////////////////////////////////
# /////////////////////////////////////////
# NOTE: not tested yet
# /////////////////////////////////////////
# /////////////////////////////////////////
# /////////////////////////////////////////
# grub theme
cd ~/Repos
git clone https://github.com/catppuccin/grub.git && cd grub
sudo cp -r src/* /usr/share/grub/themes/
GRUB_THEME="/usr/share/grub/themes/catppuccin-mocha-grub-theme/theme.txt"

sudo sed -i "s|^GRUB_THEME=\".*\"|GRUB_THEME=\"$GRUB_THEME\"|" /etc/default/grub

if [ $? -eq 0 ]; then
    echo "GRUB_THEME updated successfully."
else
    echo "Failed to update GRUB_THEME."
fi
# Update GRUB configuration
sudo grub-mkconfig -o /boot/grub/grub.cfg







# finished and delete all tempo files
cd ~/Repos
sudo rm -rf *
cd
