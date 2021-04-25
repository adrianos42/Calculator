#!/usr/bin/bash

#idl client --path=plugins/calc_manager
flutter build linux --verbose --release
rm -rf ~/opt/calculator
mkdir -p ~/opt/calculator
cp -r build/linux/x64/release/bundle/* ~/opt/calculator/
cp assets/calc.svg ~/opt/calculator/icon.svg
cp linux_package/calculator.desktop ~/.local/share/applications/calculator.desktop
echo "" >> ~/.local/share/applications/calculator.desktop
echo "Exec=$HOME/opt/calculator/calculator" >> ~/.local/share/applications/calculator.desktop
echo "Icon=$HOME/opt/calculator/icon.svg" >> ~/.local/share/applications/calculator.desktop