# What is this?
This is a program that takes multiple images as input and generates a base image and a LUT texture for each input image, allowing you to save some space for pixel art in your game.

# Why i made this?
I wanted to use Universal [LPC Spritesheet Character Generator](https://github.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator/tree/master/spritesheets)] but every variation of the same sprite is repeated, making the size colossal.

# How to use?
+ Compile the binary -> You must use Cargo build --release
+ run the binary with the arguments: 
  - program.exe --input ./assets/input/example_1 --output ./assets/output/   ## This command turns every image inside the folder into a LUT texture and generates a base to swapp colors
  - program.exe --base ./assets/output/base.png --lut-folder ./assets/output/lut/ --out-folder ./assets/output/sprite/    ## this command does the inverse. takes LUT textures and applies it to the base, generates the final image

# Requirements
+ Rust to compile the program