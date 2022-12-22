use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
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

    let out_dir = Path::new(&args.out_dir);
    let out_file_name = {
        let mut file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
        file_name.push_str(".rb");
        file_name
    };
    let out_path = out_dir.join(out_file_name);
    std::fs::create_dir_all(out_dir).unwrap();
    let mut file = File::create(out_path).unwrap();
    file.write_all(ruby_code.as_bytes()).unwrap();

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
