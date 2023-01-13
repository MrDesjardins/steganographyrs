use clap::command;
use clap::Parser;

/// CLI arguments
///
/// The command line provided in this Cargo accepts many options to encrypt a message
/// and decrypt a message. The full list of options are available in the `CliData` struct.
///
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CliData {
    /// Message to insert into the image
    ///
    /// Only pertinent when injecting a text into an image, not when decrypting (reading back)
    #[arg(short, long)]
    message: Option<String>,

    /// Password (when provided) is used to encrypt the `message` String of the `CliData`
    /// When not provided, the message is encrypted into the image without symmetric encryption
    /// of the content
    #[arg(short, long)]
    password: Option<String>,

    /// The source of the image to inject the `message`
    #[arg(short, long)]
    input_image_path: Option<String>,

    /// The input image is not altered, instead, it is copied and a copy with the message is
    /// saved at the `output_image_path` destination.
    #[arg(short, long)]
    output_image_path: Option<String>,

    /// Encrypt or decrypt
    /// When true, it inserts the `message` into a copy of the  image `input_image_path`
    ///     and output it in `output_image_path`
    /// When false, it retrieves the message from the `input_image_path`
    #[arg(short, long)]
    encrypt_mode: Option<bool>,
}

/// Options to start the steganography into encrypt or decrypt
#[derive(Clone)]
pub enum SteganographyOption {
    Encrypt(SteganographyEncryptOption),
    Decrypt(SteganographyDecryptOption),
}

/// Required options for the encryption (text to image)
#[derive(Clone)]
pub struct SteganographyEncryptOption {
    pub message: String,
    pub password: Option<String>,
    pub input_image_path: String,
    pub output_image_path: String,
}

/// Required options for the decryption (image to text)
#[derive(Clone)]
pub struct SteganographyDecryptOption {
    pub password: Option<String>,
    pub input_image_path: String,
}

/// Extract from the command line (CLI) argument the option.
/// Depending of the field *encrypt_mode*, the function returns
/// the proper formed structure or panic telling what argument
/// is missing
///
/// # Arguments
/// args - The command line argument that may contain encrypt or decrypt information
///
/// # Returns
/// Return a well formed structure for the task asked or return a failure with the missing
/// fields
pub fn extract_options(
    args: CliData,
    piped_message: Option<String>,
) -> Result<SteganographyOption, String> {
    Ok(match args.encrypt_mode {
        Some(i) => match i {
            true => {
                let message = piped_message.unwrap_or_else(|| {
                    args.message
                        .unwrap_or_else(|| panic!("Message is required"))
                });
                SteganographyOption::Encrypt({
                    SteganographyEncryptOption {
                        message: message,
                        password: args.password,
                        input_image_path: args
                            .input_image_path
                            .unwrap_or_else(|| panic!("Input image path")),
                        output_image_path: args
                            .output_image_path
                            .unwrap_or_else(|| panic!("Output image path is required")),
                    }
                })
            }
            false => SteganographyOption::Decrypt({
                SteganographyDecryptOption {
                    password: args.password,
                    input_image_path: args
                        .input_image_path
                        .unwrap_or_else(|| panic!("Input image is required")),
                }
            }),
        },
        None => panic!("Encrypt mode is required"),
    })
}
