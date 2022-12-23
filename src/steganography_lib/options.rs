use clap::command;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CliData {
    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long)]
    password: Option<String>,

    #[arg(short, long)]
    input_image_path: Option<String>,

    #[arg(short, long)]
    output_image_path: Option<String>,

    #[arg(short, long)]
    encrypt_mode: Option<bool>,
}

#[derive(Clone)]
pub struct CliOptions<'a> {
    pub password: Option<&'a str>,
    pub input_image_path: Option<&'a str>,
    pub output_image_path: Option<&'a str>,
    pub encrypt_mode: Option<bool>,
}

#[derive(Clone)]
pub enum SteganographyOption {
    Encrypt(SteganographyEncryptOption),
    Decrypt(SteganographyDecryptOption),
}

#[derive(Clone)]
pub struct SteganographyEncryptOption {
    pub message: String,
    pub password: String,
    pub input_image_path: String,
    pub output_image_path: String,
}

#[derive(Clone)]
pub struct SteganographyDecryptOption {
    pub password: String,
    pub input_image_path: String,
}

pub fn extract_options(args: CliData) -> Result<SteganographyOption, String> {
    Ok(match args.encrypt_mode {
        Some(i) => match i {
            true => SteganographyOption::Encrypt({
                SteganographyEncryptOption {
                    message: args
                        .message
                        .unwrap_or_else(|| panic!("Message is required")),
                    password: args.password.unwrap_or("default".to_string()),
                    input_image_path: args
                        .input_image_path
                        .unwrap_or_else(|| panic!("Input image path")),
                    output_image_path: args
                        .output_image_path
                        .unwrap_or_else(|| panic!("Output image path is required")),
                }
            }),
            false => SteganographyOption::Decrypt({
                SteganographyDecryptOption {
                    password: args.password.unwrap_or("default".to_string()),
                    input_image_path: args
                        .input_image_path
                        .unwrap_or_else(|| panic!("Input image is required")),
                }
            }),
        },
        None => panic!("Encrypt mode is required"),
    })
}
