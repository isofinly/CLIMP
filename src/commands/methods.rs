use image::io::Reader as ImageReader;
use image::{imageops, ImageFormat};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::io;
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

use crate::ascii::{from_str, render, render_to_file, RenderOptions};

use super::Args;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(&img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));

    resize(&small, old_dims)
}

impl Args {
    /// Pixelates the image based via `resize` function
    ///
    /// If there's an error then `resize` function will print problematic pixels
    pub fn pixelate(&mut self) -> Result<(), Box<dyn Error>> {
        let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
        let img_result = pixelate(
            &img,
            (
                self.get_pixel_size().unwrap(),
                self.get_pixel_size().unwrap(),
            ),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Pixelated image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

fn blur(img: &Image, radius: u32) -> Image {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
                "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
            ]),
    );
    pb.set_message("\x1b[33mBlurring...\x1b[0m");
    let img = imageops::blur(img, radius as f32);
    pb.finish_with_message("\x1b[32mDone\x1b[0m");
    img
}

impl Args {
    /// Blurs the image via standard gaussian blur
    ///
    /// It is not recommended to use large values for blur `radius` as the method complexity is not constant
    pub fn blur(&mut self) -> Result<(), Box<dyn Error>> {
        let img_result = blur(
            &ImageReader::open(self.get_filepath().clone())?
                .decode()?
                .into_rgba8(),
            self.get_radius().unwrap(),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Blurred image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

impl Args {
    /// Flips the image horizontally
    pub fn mirror(&mut self) -> Result<(), Box<dyn Error>> {
        let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
        let img_result = img.fliph();
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Mirrored image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

impl Args {
    /// Flips the image vertically
    pub fn flip_vertical(&mut self) -> Result<(), Box<dyn Error>> {
        let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
        let img_result = img.flipv();
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Flipped image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

fn rotate(img: &Image) -> Image {
    imageops::rotate90(img)
}

impl Args {
    /// Rotates the image by fixed amount of 90 degrees
    pub fn rotate(&mut self) -> Result<(), Box<dyn Error>> {
        let img_result = rotate(
            &ImageReader::open(self.get_filepath().clone())?
                .decode()?
                .into_rgba8(),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Rotated image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

fn grayscale(img: &Image) -> Image {
    let (width, height) = img.dimensions();
    let mut img_buf = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let red = pixel[0] as f32;
        let green = pixel[1] as f32;
        let blue = pixel[2] as f32;

        let grayscale_value = ((red + green + blue) / 3.0) as u8;

        let grayscale_pixel = Rgba([grayscale_value, grayscale_value, grayscale_value, 255]);

        img_buf.put_pixel(x, y, grayscale_pixel);
    }
    img_buf
}

impl Args {
    /// Grayscales the image
    pub fn grayscale(&mut self) -> Result<(), Box<dyn Error>> {
        let img_result = grayscale(
            &ImageReader::open(self.get_filepath().clone())?
                .decode()?
                .into_rgba8(),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Grayscale image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

fn monochrome_ugly(img: &Image, threshold: f32) -> Image {
    let (width, height) = img.dimensions();
    let mut img_buf = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let red = pixel[0] as f32;
        let green = pixel[1] as f32;
        let blue = pixel[2] as f32;

        let luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue;

        let threshold = threshold;
        let new_pixel = if luminance >= threshold {
            Rgba([255, 255, 255, 255])
        } else {
            Rgba([0, 0, 0, 255])
        };

        img_buf.put_pixel(x, y, new_pixel);
    }
    img_buf
}

impl Args {
    /// Monochromes the image into black and white regions based on luminance
    ///
    /// `Threshold` defines the luminance threshold for black and white
    pub fn monochrome_ugly(&mut self) -> Result<(), Box<dyn Error>> {
        let img_result = monochrome_ugly(
            &ImageReader::open(self.get_filepath().clone())?
                .decode()?
                .into_rgba8(),
            self.get_threshold().unwrap(),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Monochrome image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

fn resize(img: &Image, new_dims: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dims;

    let mut resized = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("({old_x} -> {new_x}, {old_y} -> {new_y})");
        }
    }

    resized
}

impl Args {
    /// Resizes the image to the given dimensions.
    ///
    /// If the image is larger than the `new_dims`,
    /// it will be cropped and missed pixels will be printed
    pub fn resize(&mut self) -> Result<(), Box<dyn Error>> {
        let img = ImageReader::open(self.get_filepath().clone())?.decode()?;

        let img_result = resize(
            &img.into_rgba8(),
            (self.get_resize().unwrap(), self.get_resize().unwrap()),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Scaled image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

impl Args {
    pub fn ascii(&mut self, flag: bool) -> Result<(), Box<dyn Error>> {
        let clusters =
            UnicodeSegmentation::graphemes(self.get_charset().as_str(), true).collect::<Vec<_>>();
        let charset = from_str(self.get_charset().as_str()).unwrap_or(clusters.as_slice());

        let path = self.get_filepath().clone();

        if flag {
            render(
                path.to_str().unwrap(),
                &mut io::stdout(),
                &RenderOptions {
                    width: self.get_width().or(Some(80)),
                    height: self.get_height(),
                    colored: self.is_colored(),
                    invert: self.is_invert(),
                    charset,
                },
            )?;
        } else {
            render_to_file(
                path.to_str().unwrap(),
                self.get_output_name(),
                &RenderOptions {
                    width: self.get_width().or(Some(80)),
                    height: self.get_height(),
                    colored: self.is_colored(),
                    invert: self.is_invert(),
                    charset,
                },
            )?;
        }
        Ok(())
    }
}

fn curse(img: &Image) -> Image {
    let stretch_x = 1.5;
    let stretch_y = 0.5;

    let (old_width, old_height) = img.dimensions();

    let new_width = (old_width as f32 * stretch_x) as u32;
    let new_height = (old_height as f32 * stretch_y) as u32;

    let mut cursed = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in cursed.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("({old_x} -> {new_x}, {old_y} -> {new_y})");
        }
    }

    pixelate(&DynamicImage::ImageRgba8(cursed), (5, 5))
}

impl Args {
    /// Curses the image stretching and squishing it by 1.5 & 0.5 correspondingly
    ///
    /// After that pixelate the image with pixel size of 5
    pub fn curse(&mut self) -> Result<(), Box<dyn Error>> {
        let img_result = curse(
            &ImageReader::open(self.get_filepath().clone())?
                .decode()?
                .into_rgba8(),
        );
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("Cursed image saved as {:?}", self.get_output_name());
        Ok(())
    }
}

fn zxc(img: &Image) -> Image {
    let mid_curse: ImageBuffer<Rgba<u8>, Vec<u8>> = curse(img);
    monochrome_ugly(&mid_curse, 125.0)
}

impl Args {
    /// ZXC the image. Ultimate dead inside happens here.
    ///
    /// Wish you the worst of luck.
    pub fn zxc(&mut self) -> Result<(), Box<dyn Error>> {
        let img_result = zxc(&ImageReader::open(self.get_filepath().clone())?
            .decode()?
            .into_rgba8());
        let _ = img_result.save_with_format(
            self.get_output_name(),
            ImageFormat::from_extension(self.get_output_ext().as_deref().unwrap())
                .unwrap_or(ImageFormat::Jpeg),
        );
        println!("ZXCursed image saved as {:?}", self.get_output_name());
        Ok(())
    }
}
