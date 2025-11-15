use clap::{Parser, Subcommand};
use afptool_rs::{unpack_file, pack_rkfw, pack_rkaf};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "afptool-rs")]
#[command(about = "A Rust tool for packing and unpacking RockChip firmware images")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Unpack {
        #[arg(help = "Path to the firmware file (RKFW or RKAF format)")]
        input: String,

        #[arg(help = "Directory where extracted files will be saved")]
        output: String,
    },

    PackRkfw {
        #[arg(help = "Directory containing BOOT and embedded-update.img files")]
        input: String,

        #[arg(help = "Output RKFW firmware image file path")]
        output: String,

        #[arg(short, long, help = "Chip family (e.g., RK29XX, RK30XX, RK31XX, RK32XX, RK3368, RK3326, RK3562, RK3566, PX30)")]
        chip: String,

        #[arg(short, long, help = "Version in format: major.minor.build (e.g., 8.1.0)")]
        version: String,

        #[arg(short, long, help = "Unix timestamp for build date (e.g., 1731031994)")]
        timestamp: i64,

        #[arg(long, help = "Code field as hex string (e.g., 0x02000000)")]
        code: String,
    },

    PackRkaf {
        #[arg(help = "Directory containing package-file and files to pack")]
        input: String,

        #[arg(help = "Output RKAF update image file path")]
        output: String,

        #[arg(short, long, help = "Model name")]
        model: String,

        #[arg(short = 'M', long, help = "Manufacturer name")]
        manufacturer: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Unpack { input, output } => {
            unpack_file(&input, &output)?;
        }
        Commands::PackRkfw{ input, output, chip, version, timestamp, code } => {
            pack_rkfw(&input, &output, &chip, &version, timestamp, &code)?;
        }
        Commands::PackRkaf { input, output, model, manufacturer } => {
            pack_rkaf(&input, &output, &model, &manufacturer)?;
        }
    }

    Ok(())
}