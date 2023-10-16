use super::image_renderer::ImageRenderer;
use super::renderer::RenderOptions;
use super::renderer::Renderer;
use image::DynamicImage;
use image::ImageResult;
use std::{io, path::Path};

pub fn render_to_file<P1: AsRef<Path>, P2: AsRef<Path>>(
    input_path: P1,
    output_path: P2,
    options: &RenderOptions<'_>,
) -> ImageResult<()> {
    let image = image::open(input_path)?;
    let mut output_file = io::BufWriter::new(std::fs::File::create(output_path)?);
    render_image(&image, &mut output_file, options)?;
    Ok(())
}

pub fn render<P: AsRef<Path> + AsRef<str>>(
    path: P,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let image = &image::open(path)?;
    render_image(image, to, options)
}

pub fn render_image(
    image: &DynamicImage,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = ImageRenderer::new(image, options);
    renderer.render_to(to)?;
    Ok(())
}

#[allow(dead_code)]
pub fn render_to<P: AsRef<Path> + AsRef<str>>(
    path: P,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let image = &image::open(path)?;
    let renderer = ImageRenderer::new(image, options);
    renderer.render(buffer)?;
    Ok(())
}

#[allow(dead_code)]
pub fn render_image_to(
    image: &DynamicImage,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = ImageRenderer::new(image, options);
    renderer.render(buffer)?;
    Ok(())
}
