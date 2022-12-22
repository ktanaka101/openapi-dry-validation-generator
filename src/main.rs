use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;

use openapi_dry_validation_generator::{
    generate_dry_validation_from_json, generate_dry_validation_from_yaml,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    spec: String,

    #[arg(short, long, default_value = "out")]
    out_dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.spec);
    let file_content = {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        buf
    };

    let ruby_code = match select_file_type(path)? {
        SupportFileType::Json => generate_dry_validation_from_json(&file_content),
        SupportFileType::Yaml => generate_dry_validation_from_yaml(&file_content),
    };

    let output = Output::new(&args.out_dir, path).unwrap();
    output.create_dir_all().unwrap();
    output.write_file_all(&ruby_code).unwrap();

    Ok(())
}

enum SupportFileType {
    Json,
    Yaml,
}

fn select_file_type(path: &Path) -> Result<SupportFileType> {
    match path.extension() {
        Some(extension) => match extension.to_ascii_lowercase().to_str().unwrap() {
            "json" => Ok(SupportFileType::Json),
            "yaml" | "yml" => Ok(SupportFileType::Yaml),
            ext => anyhow::bail!("Unsupported file extension.(ext: {ext})"),
        },
        None => anyhow::bail!("Unknown file extension."),
    }
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
