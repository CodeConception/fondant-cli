use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert from .fon font to .png tilemap
    #[command(name = "to-tilemap", visible_alias = "totilemap")]
    ToTilemap {
        /// Input .fon font file
        #[arg(short, long, value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Output .png tilemap file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Positional arguments: [input] [output]
        #[arg(value_name = "FILES")]
        files: Vec<PathBuf>,
    },

    /// Convert from tilemap to font
    #[command(name = "to-font", visible_alias = "tofont")]
    ToFont {
        /// Input .png tilemap file
        #[arg(short, long, value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Output .fon font file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Positional arguments: [input] [output]
        #[arg(value_name = "FILES")]
        files: Vec<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ToTilemap {
            input,
            output,
            files,
        } => {
            // If flags are provided, use them or use positional arguments if they exist
            let (input, output) = if let (Some(input), Some(output)) = (input, output) {
                (input, output)
            } else if files.len() == 2 {
                (files[0].clone(), files[1].clone())
            } else {
                eprintln!("Error: Please provide input and output files either as flags or positional arguments");
                std::process::exit(1);
            };

            println!("Converting font to tilemap: {:?} -> {:?}", input, output);

            let input_bytes: Vec<u8> = std::fs::read(input)
                .context("Failed to read input file")
                .unwrap();

            let font = fondant::Font::from_bytes(&input_bytes);
        }
        Commands::ToFont {
            input,
            output,
            files,
        } => {
            let (input, output) = if let (Some(input), Some(output)) = (input, output) {
                (input, output)
            } else if files.len() == 2 {
                (files[0].clone(), files[1].clone())
            } else {
                eprintln!("Error: Please provide input and output files either as flags or positional arguments");
                std::process::exit(1);
            };

            println!("Converting tilemap to font: {:?} -> {:?}", input, output);
            // Add your conversion logic here
        }
    }
}
