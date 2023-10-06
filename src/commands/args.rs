use std::path::PathBuf;
/// Command line arguments that can be parsed from the command line.
/// Implemented in @flag_matcher.
/// Formats output filename via `format_output_name` function.
pub struct Args {
    filepath: PathBuf,
    radius: Option<u32>,
    pixel: Option<u32>,
    resize: Option<u32>,
    output: Option<PathBuf>,
    file_ext: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    colored: bool,
    invert: bool,
    charset: String,
    threshold: Option<f32>,
}

#[allow(dead_code)]
impl Args {
    pub fn new() -> Self {
        Args {
            filepath: PathBuf::from(""),
            radius: None,
            pixel: None,
            output: None,
            resize: None,
            file_ext: None,
            width: None,
            height: None,
            colored: false,
            invert: false,
            charset: "default".to_string(),
            threshold: None,
        }
    }

    pub fn get_filepath(&self) -> &PathBuf {
        &self.filepath
    }

    pub fn get_radius(&self) -> Option<u32> {
        self.radius
    }

    pub fn get_pixel(&self) -> Option<u32> {
        self.pixel
    }

    pub fn get_resize(&self) -> Option<u32> {
        self.resize
    }

    pub fn get_output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
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

    pub fn set_filepath(&mut self, filepath: PathBuf) {
        self.filepath = filepath;
    }

    pub fn set_radius(&mut self, radius: Option<u32>) {
        self.radius = radius;
    }

    pub fn set_pixel(&mut self, pixel: Option<u32>) {
        self.pixel = pixel;
    }

    pub fn set_resize(&mut self, resize: Option<u32>) {
        self.resize = resize;
    }

    pub fn set_output(&mut self, output: Option<PathBuf>) {
        self.output = output;
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

    pub fn format_output_name(&self) -> PathBuf {
        let output_name = self
            .output
            .as_ref()
            .unwrap_or(&self.filepath)
            .with_extension(self.file_ext.as_deref().unwrap_or("jpg"));
        output_name
    }
}
