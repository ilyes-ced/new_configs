#!/bin/bash
set -e



if ! command -v sudo &>/dev/null; then
  echo "sudo not found. Install sudo first."
  exit 1
fi

echo "Make sure multilib is enabled in /etc/pacman.conf before continuing."
read -rp "Press ENTER to continue or Ctrl+C to abort..."




# install yay
if ! command -v yay &>/dev/null; then
  echo "Installing yay..."
  git clone https://aur.archlinux.org/yay.git /tmp/yay
  (cd /tmp/yay && makepkg -si --noconfirm)
  rm -rf /tmp/yay
fi


ask() {
  local prompt="$1"
  read -rp "$prompt [y/N]: " reply
  [[ "$reply" =~ ^[Yy]$ ]]
}


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


for dir in ~/Repos ~/Installs ~/Projects; do
  [ -d "$dir" ] || mkdir "$dir"
done




############################################
############################################
############################################
# installing pacman packages
############################################
############################################
############################################
echo "Installing pacman packages ..."
sudo pacman -S --noconfirm hyprland mpv neovim playerctl alacritty rofi atuin zellij zsh tldr go git base-devel curl wget zsh playerctl udiskie grim hyprpicker cliphist cmake meson ttf-jetbrains-mono ttf-jetbrains-mono-nerd adobe-source-han-sans-jp-fonts fastfetch bat waybar python-pywal python-colorthief nvim 
echo "All pacman packages installed successfully!"



# nvidia stuff
if ask "Install Nvidia drivers?"; then
    sudo pacman -S --needed --noconfirm nvidia lib32-nvidia-utils nvidia-utils nvidia-settings
else
    echo "Skipping nvidia drivers."
fi

# gaming stuff
# read -rp "Install gaming tools (Steam, Wine, Lutris, etc.)? [y/N]: " install_gaming
# if [[ "$install_gaming" =~ ^[Yy]$ ]]; then
#     sudo pacman -S --needed --noconfirm wine wine-mono wine-gecko winetricks lib32-libpulse steam lutris
# else
#   echo "Skipping gaming tools."
# fi
#? needs to enable multilib
# enabling multilib


if ask "Install gaming tools (Steam, Wine, Lutris)?"; then
  if grep -q '^\s*#\s*\[multilib\]' /etc/pacman.conf; then
    sudo sed -i '/^\s*#\s*\[multilib\]/,/^\s*#\s*Include/ {
      s/^\s*#\s*//
    }' /etc/pacman.conf
  fi
  sudo pacman -Syu --needed --noconfirm wine wine-mono wine-gecko winetricks lib32-libpulse steam lutris
else
    echo "Skipping gaming tools."
fi



############################################
############################################
############################################
# installing yay packages
############################################
############################################
############################################
echo "Installing yay packages from the list..."
yay -S --needed --noconfirm brave-bin copyq floorp-bin zen-browser-bin vscodium-bin wlogout noto-fonts-ar
echo "All yay packages installed successfully!"





### ================================
### rust install
### ================================
if ! command -v rustc &>/dev/null; then
  echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
fi





### ================================
### ZSH + OH-MY-ZSH
### ================================
if [[ "$SHELL" != *zsh ]]; then
  echo "Changing default shell to zsh (password required)"
  chsh -s "$(which zsh)"
fi

if [[ ! -d "$HOME/.oh-my-zsh" ]]; then
  echo "Installing Oh My Zsh..."
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
fi




### ================================
### ZSH PLUGINS
### ================================
ZSH_CUSTOM="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}"

git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions

git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting


echo "adding the plugins to the config file"
ZSHRC="$HOME/.zshrc"
PLUGINS_TO_ADD=(zsh-autosuggestions zsh-syntax-highlighting)
for plugin in "${PLUGINS_TO_ADD[@]}"; do
  if ! grep -q "plugins=.*\b$plugin\b" "$ZSHRC"; then
    sed -i "/^plugins=(/ s/)/ $plugin)/" "$ZSHRC"
  fi
done
echo "plugins=(... zsh-autosuggestions zsh-syntax-highlighting)"



### ================================
### starship
### ================================
curl -sS https://starship.rs/install.sh | sh
echo 'eval "$(starship init zsh)"' >> ~/.zshrc




### ================================
### ATUIN
### ================================
if ! command -v atuin &>/dev/null; then
  echo "Installing atuin..."
  curl --proto '=https' --tlsv1.2 -LsSf https://setup.atuin.sh | sh
  echo 'eval "$(atuin init zsh)"' >> ~/.zshrc
fi

### ================================
### ZELLIJ
### ================================
if ! command -v cargo &>/dev/null; then
  echo "Rust not fully installed yet."
  echo "Run rustup manually if needed."
else
  cargo install --locked zellij || true
  echo 'eval "$(zellij setup --generate-auto-start zsh)"' >> ~/.zshrc
fi






### ================================
### GIT CONFIG
### ================================
# echo "Configure git:"
# read -rp "Git email: " GIT_EMAIL
# read -rp "Git name: " GIT_NAME
# 
# git config --global user.email "$GIT_EMAIL"
# git config --global user.name "$GIT_NAME"
git config --global user.name "ilyes-ced"
git config --global user.email "random_dude_233@proton.me"





############################################
############################################
############################################
# installing firefox secure script
############################################
############################################
############################################
# cd ~/Repos
# git clone https://github.com/simeononsecurity/FireFox-Privacy-Script --depth 1
# cd FireFox-Privacy-Script
# sudo chmod +x ./sos-firefoxprivacy.sh
# sudo bash ./sos-firefoxprivacy.sh
# cd




############################################
############################################
############################################
# installing vscode extention + setting the config file
############################################
############################################
############################################
extensions=(
    "chunsen.bracket-select"
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






echo "Copying config files . . ."
cp -a .config/. ~/.config/
echo "Finished copying config files . . ."




echo "All extensions have been installed."