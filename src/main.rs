use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;

use openapi_dry_validation_generator::generate_dry_validation_from_root_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "out")]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ruby_code = generate_dry_validation_from_root_file(&args.input);

    let output = Output::new(&args.output, &args.input).unwrap();
    output.create_dir_all().unwrap();
    output.write_file_all(&ruby_code).unwrap();

    Ok(())
}

struct Output {
    file_path: PathBuf,
}
impl Output {
    fn new<S1, S2>(output_dir: &S1, input_file_path: &S2) -> Result<Self>
    where
        S1: AsRef<std::ffi::OsStr> + ?Sized,
        S2: AsRef<std::ffi::OsStr> + ?Sized,
    {
        let mut input_file_name = Path::new(input_file_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        input_file_name.push_str(".rb");
        let file_path = Path::new(output_dir).join(input_file_name);

        Ok(Self { file_path })
    }

    fn create_dir_all(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.file_path.parent().unwrap())
    }

    fn write_file_all(&self, content: &str) -> std::io::Result<()> {
        let mut file = File::create(&self.file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }
}
