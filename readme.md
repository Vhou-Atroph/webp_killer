# webp_killer

This is a program created for the sole purpose of converting .webp files into any other (better) image format from the command line. I don't like webp files, and I know that lots of others don't either. Hopefully this program will help you deal with your hatred of this menace. Once the webp file is converted, it is then removed from existence.  
This program was built in Rust as a fun little project- I'm trying to learn it and this felt like a neat thing to do with it.

## usage

### To convert

#### Cargo

To run this from cargo, you must have a .webp file in the root of the project. Once this is achieved, run the following in your favorite terminal:

```powershell
cargo run [webp file] [desired extension]
```

For example, if I wanted to convert the file `flippy.webp` to a png file, I would do the following:

```powershell
cargo run flippy png
```

Note that you do not actually need to add the .webp extension to your file name, nor do you need to specify your desired extension if it is png- `cargo run flippy` would produce the same results as the command featured above.

#### Binary

To run this from the binary, place it in the same directory as any webp file you want to convert, and run the following in your favorite terminal:

```powershell
webp_killer.exe [webp file] [desired extension]
```

For example, if I wanted to convert the file `flippy.webp` to a png file, I would do the following:

```powershell
webp_killer.exe flippy png
```

Just like with cargo, you do not need to specify the extension if you want to convert your webp file into a png file. `webp_killer.exe flippy` works the exact same way as the command featured above.

## To Detect and Destroy

Alternatively, you can just run the binary file in command prompt without any arguments, and the program will automatically detect webp files downloaded to the directory and convert them immediately.

```powershell
webp_killer.exe
```

## final

Thanks for checking out my tiny little project, I hope your days in the future will be filled with less webp file struggles :)
