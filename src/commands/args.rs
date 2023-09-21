use std::path::PathBuf;

pub struct Args {
    pub filepath: PathBuf,
    pub radius: Option<u32>,
    pub pixel: Option<u32>,
    pub resize: Option<u32>,
    pub output: Option<PathBuf>,
    pub file_ext: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub colored: bool,
    pub invert: bool,
    pub charset: String,
}

impl Args {
    pub fn format_output_name(&self) -> PathBuf {
        let output_name = self
            .output
            .as_ref()
            .unwrap_or(&self.filepath)
            .with_extension(self.file_ext.as_deref().unwrap_or("jpg"));
        output_name
    }

}


