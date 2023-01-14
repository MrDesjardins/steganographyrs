use clap;
use clap::builder::TypedValueParser;
use clap::command;
use clap::Parser;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    Inject,
    Extract,
    Encrypt,
    Decrypt,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Inject => "inject",
            Self::Extract => "extract",
            Self::Encrypt => "encrypt",
            Self::Decrypt => "decrypt",
        };
        s.fmt(f)
    }
}
impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inject" => Ok(Self::Inject),
            "extract" => Ok(Self::Extract),
            "encrypt" => Ok(Self::Encrypt),
            "decrypt" => Ok(Self::Decrypt),
            _ => Err(format!("Unknown mode: {s}")),
        }
    }
}

/// CLI arguments
///
/// The command line provided in this Cargo accepts many options to encrypt a message
/// and decrypt a message. The full list of options are available in the `CliData` struct.
///
#[derive(Parser)]
#[clap(name = "from_str")]
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

    /// Possible values:
    /// "inject"= inject the message into an image.
    /// "extract" = extract from an image the message.
    /// "encrypt" = encrypt the message without using any image (not steganography related, utility feature).
    /// "decrypt" = decrypt a message withotu using any image  (not steganography related, utility feature).
    #[arg(short='e', long, value_parser = clap::builder::PossibleValuesParser::new(["inject", "extract", "encrypt", "decrypt"])
    .map(|s| s.parse::<Mode>().unwrap()),)]
    mode: Option<Mode>,
}

/// Options to start the steganography into encrypt or decrypt
#[derive(Clone)]
pub enum SteganographyOption {
    InjectMessageIntoImage(SteganographyInjectOption),
    ExtractMessageFromImage(SteganographyExtractOption),
}

/// Required options for the injection (text to image)
#[derive(Clone)]
pub struct SteganographyInjectOption {
    pub message: String,
    pub password: Option<String>,
    pub input_image_path: String,
    pub output_image_path: String,
}

/// Required options for the extraction (image to text)
#[derive(Clone)]
pub struct SteganographyExtractOption {
    pub password: Option<String>,
    pub input_image_path: String,
}

/// Extract from the command line (CLI) argument the option.
/// Depending of the mode, the function returns
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
    Ok(match args.mode {
        Some(i) => match i {
            Mode::Inject => {
                let message = piped_message.unwrap_or_else(|| {
                    args.message
                        .unwrap_or_else(|| panic!("Message is required"))
                });
                SteganographyOption::InjectMessageIntoImage({
                    SteganographyInjectOption {
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
            Mode::Extract => SteganographyOption::ExtractMessageFromImage({
                SteganographyExtractOption {
                    password: args.password,
                    input_image_path: args
                        .input_image_path
                        .unwrap_or_else(|| panic!("Input image is required")),
                }
            }),
            Mode::Encrypt => todo!(""),
            Mode::Decrypt => todo!(""),
        },
        None => panic!("Encrypt mode is required"),
    })
}
