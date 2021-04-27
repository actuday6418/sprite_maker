# sprite_maker
Make .sprite files from PNG images. You can use these sprites with the github.com/actuday6418/pixel game utility. Head over there for a detailed explanation of the sprite format.

# Dependencies
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Python3](https://www.python.org/downloads/)
3. [Tkdnd](https://github.com/pmgagne/tkinterdnd2)

# Source code
The backend (image transforms and file handling) is done by a dynamic library written in Rust. An extension for Tkinter is used as the frontend.

# Building
1. ui.py has an external dependency that first has to be met.
```
   cd frontend
   git clone https://github.com/pmgagne/tkinterdnd2
   cd ..
```
2. Simply run the build script
```
   sh build.sh
```

# Running
Execute sprite_maker. It's a one line script!
```
    ./sprite_maker
```

# Usage
I solemnly acknowledge the low quality and artifacts in the screen recording.

Generating the .sprite file -
![shot](https://user-images.githubusercontent.com/56124831/99193560-cc8ce080-279f-11eb-968f-eac348aeb787.gif)

Using the file in your app (This looks mush better than the recording makes it out to be. Github will only host so much.) - 
![op](https://user-images.githubusercontent.com/56124831/99193554-c7c82c80-279f-11eb-9ff8-a4b14ed8c62d.gif)
