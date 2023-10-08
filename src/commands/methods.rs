use image::imageops;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Rotates the image by fixed amount of 90 degrees
pub fn rotate(img: &Image) -> Image {
    let out_img = imageops::rotate90(img);
    out_img
}

/// Resizes the image to the given dimensions.
///
/// If the image is larger than the `new_dims`,
/// it will be cropped and missed pixels will be printed
pub fn resize(img: &Image, new_dims: (u32, u32)) -> Image {
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

/// Pixelates the image based via `resize` function
///
/// If there's an error then `resize` function will print problematic pixels
pub fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(&img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));

    let pixelated = resize(&small, old_dims);
    pixelated
}

/// Blurs the image via standard gaussian blur
///
/// It is not recommended to use large values for blur `radius` as the method complexity is not constant
pub fn blur(img: &Image, radius: u32) -> Image {
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
/// Monochromes the image into black and white regions based on luminance
///
/// `Threshold` defines the luminance threshold for black and white
pub fn monochrome_ugly(img: &Image, threshold: f32) -> Image {
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

/// Grayscales the image
pub fn grayscale(img: &Image) -> Image {
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
/// Curses the image stretching and squishing it by 1.5 & 0.5 correspondingly
/// 
/// After that pixelate the image with pixel size of 5
pub fn curse(img: &Image) -> Image {
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
     
    pixelate(&DynamicImage::ImageRgba8(cursed), (5,5))
    
}


/// ZXC the image. Ultimate dead inside happens here. 
/// 
/// Wish you the worst of luck.
pub fn zxc(img: &Image) -> Image {
    let mid_curse: ImageBuffer<Rgba<u8>, Vec<u8>> = curse(img);
    monochrome_ugly(&mid_curse, 125.0)
}
