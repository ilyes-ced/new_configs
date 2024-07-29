#!/bin/bash

############################################
############################################
############################################
# update system
############################################
############################################
############################################
sudo pacman -S archlinux-keyring --noconfirm
sudo pacman -Syu --noconfirm
# needed to isntall vecodium at the time of writing this
sudo pacman -S debugedit
mkdir Repos Installs



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
while read -r package; do
    echo "Installing $package..."
    sudo pacman -S --noconfirm "$package"
done < "$package_list_file"

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
while read -r package; do
    echo "Installing $package..."
    yay -S --noconfirm "$package"
done < "$package_list_file"

echo "All yay packages installed successfully!"








############################################
############################################
############################################
# installing firefox secure script
############################################
############################################
############################################
cd Repos
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

vscodium_extentions="vscodium_extentions.txt"

# Loop through the extensions and install them
while read extension; do
  codium --install-extension "$extension"
done < "$extensions_file"

echo "Extensions installed successfully!"

# set the config file
cp vscodium/settings.json ~/.config/VSCodium/User/settings.json


















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
cd Repos 
git clone https://github.com/elkowar/eww  --depth 1
cd eww
cargo build --release --no-default-features --features x11
cp target/release/eww ~/Installs
cd ~/Installs
chmod +x ./eww
mkdir ~/.config/eww
./eww daemon
./eww open <window_name>



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
# add blackarch repo + maybe most useful tools
# add firefox extentions


# add option to save password and use it later
# add option to select a WM or multiple


# install grub theme