
# CLIMP - Command Line Image Manipulation Processor

CLIMP is a command-line application for performing various image manipulation tasks on images. It provides a wide range of commands to edit and transform images, making it a versatile tool for your image processing needs.

## Installation

You can install CLIMP from github releases page.

## Usage

CLIMP accepts the following command-line arguments and options:

### Commands:

-   `pixelate`: Pixelate the image with a given pixel size.
-   `blur`: Blur the image with a given radius.
-   `mirror`: Mirror the image.
-   `flip_vertical`: Flip the image vertically.
-   `rotate`: Rotate an image 90 degrees clockwise.
-   `grayscale`: Make the image grayscale.
-   `monochrome_ugly`: Make the image monochrome.
-   `scale`: Scale the image.
-   `ascii`: Render the image as ASCII art with a given charset.
-   `curse`: Curse the image.
-   `zxc`: Apply the ultimate zxc dead inside the image.
-   `help`: Print this message or the help of the given subcommand(s).

### Arguments:

-   `<filepath>`: File path to the image you want to edit.

### Options:

-   `-o, --output <filepath>`: Specify the output file path. For image commands, you can set the output format to one of ImageFormat's values: Png, Jpeg, Gif, WebP, Pnm, Tiff, Tga, Dds, Bmp, Ico, Hdr, OpenExr, Farbfeld, Avif, Qoi. However, the `ascii` command produces files without any extension.
-   `-h, --help`: Print help.
-   `-V, --version`: Print version.

### Subcommands:

-   `pixelate`: Pixelate the image with a given pixel size.
    
    -   `-p, --pixel_size <VALUE>`: Pixel size for pixelation (required).
-   `blur`: Blur the image with a given radius.
    
    -   `-r, --blur_radius <VALUE>`: Blur radius (required).
-   `monochrome_ugly`: Make the image monochrome.
    
    -   `-t, --threshold <VALUE>`: Monochrome threshold value (default: 128.0).
-   `scale`: Scale the image.
    
    -   `-s, --scale <VALUE>`: Scaling factor (required).
-   `ascii`: Render the image as ASCII art with various options.
    
    -   `--width <VALUE>`: Set the width for ASCII art output.
    -   `--height <VALUE>`: Set the height for ASCII art output.
    -   `--colored <BOOL>`: Enable colored ASCII art (true or false).
    -   `--invert <BOOL>`: Invert ASCII art colors (true or false).
    -   `--charset <SET>`: Set the character set for ASCII art.
    -   `-v, --verbose_only <BOOL>`: Generate verbose ASCII art (true or false).

## Examples

Here are some examples of how to use CLIMP:

### Pixelate an Image:


`climp image.jpg -o pixelated.png pixelate -p 5` 

### Convert an Image to Grayscale:


`climp image.jpg -o grayscale.png grayscale` 

### Generate ASCII Art from an Image:


`climp imagejpg ascii --width 80 --height 40 --colored --charset russian --verbose_only` 

### Rotate an Image 90 Degrees Clockwise:

`climp image.jpg -o rotated.jpg rotate` 


## Author

Climp is developed and maintained by **Isofinly** with use of some external libraries. If you have any questions or issues, feel free to contact me :)

----------

Enjoy using CLIMP for all your image manipulation tasks!