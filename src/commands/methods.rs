use image::imageops;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn rotate(img: &Image) -> Image {
    let out_img = imageops::rotate90(img);
    out_img
}

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

pub fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(&img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));

    let pixelated = resize(&small, old_dims);
    pixelated
}

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
