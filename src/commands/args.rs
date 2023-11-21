use std::path::PathBuf;

/// Command line arguments that can be parsed from the command line.
///
/// Implemented in @flag_matcher.
///
/// Formats output filename via `format_output_name` function.
#[derive(Debug, Default)]
pub struct Args {
    filepath: PathBuf,
    radius: Option<u32>,
    pixel: Option<u32>,
    resize: Option<u32>,
    file_ext: Option<String>,
    output_name: PathBuf,
    output_ext: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    colored: bool,
    invert: bool,
    charset: String,
    threshold: Option<f32>,
    verbose_only: bool,
}

#[allow(dead_code)]
impl Args {
    pub fn new() -> Self {
        Args::default()
    }

    pub fn get_filepath(&self) -> &PathBuf {
        &self.filepath
    }

    pub fn get_radius(&self) -> Option<u32> {
        self.radius
    }

    pub fn get_pixel_size(&self) -> Option<u32> {
        self.pixel
    }

    pub fn get_resize(&self) -> Option<u32> {
        self.resize
    }

    pub fn get_file_ext(&self) -> Option<&String> {
        self.file_ext.as_ref()
    }

    pub fn get_width(&self) -> Option<u32> {
        self.width
    }

    pub fn get_height(&self) -> Option<u32> {
        self.height
    }

    pub fn is_colored(&self) -> bool {
        self.colored
    }

    pub fn is_invert(&self) -> bool {
        self.invert
    }

    pub fn get_charset(&self) -> &String {
        &self.charset
    }

    pub fn get_threshold(&self) -> Option<f32> {
        self.threshold
    }

    pub fn get_verbose_only(&self) -> bool {
        self.verbose_only
    }

    pub fn get_output_name(&self) -> &PathBuf {
        &self.output_name
    }

    pub fn get_output_ext(&self) -> Option<&String> {
        self.output_ext.as_ref()
    }

    pub fn set_filepath(&mut self, filepath: PathBuf) {
        self.filepath = filepath;
    }

    pub fn set_blur_radius(&mut self, radius: Option<u32>) {
        self.radius = radius;
    }

    pub fn set_pixel(&mut self, pixel: Option<u32>) {
        self.pixel = pixel;
    }

    pub fn set_resize(&mut self, resize: Option<u32>) {
        self.resize = resize;
    }

    pub fn set_file_ext(&mut self, file_ext: Option<String>) {
        self.file_ext = file_ext;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }

    pub fn set_colored(&mut self, colored: bool) {
        self.colored = colored;
    }

    pub fn set_invert(&mut self, invert: bool) {
        self.invert = invert;
    }

    pub fn set_charset(&mut self, charset: String) {
        self.charset = charset;
    }

    pub fn set_threshold(&mut self, threshold: Option<f32>) {
        self.threshold = threshold;
    }

    pub fn set_verbose_only(&mut self, verbose_only: bool) {
        self.verbose_only = verbose_only;
    }

    pub fn set_output_name(&mut self, output_name: PathBuf) {
        self.output_name = output_name;
    }

    pub fn set_output_ext(&mut self, output_ext: Option<String>) {
        self.output_ext = output_ext;
    }
}
