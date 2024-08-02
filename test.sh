#!/bin/bash

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






# set the config file
cp vscodium/settings.json ~/.config/VSCodium/User/settings.json


