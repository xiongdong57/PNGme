mod chunk;
mod chunk_type;
mod command;
mod png;

// use crate::command::Cli;
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::str::FromStr;

use clap;
use color_eyre::eyre::Result;
use std::fs;

use clap::{Parser, Subcommand};

/// A CLI to encode/decode message to PNG
#[derive(Parser, Debug)]
#[clap(name = "PNGme")]
#[clap(about = "A CLI to encode/decode message to PNG", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Encode message to PNG
    #[clap(arg_required_else_help = true)]
    Encode {
        /// PNG file path
        path: String,
        /// chunckType: four letters
        chunk_type: String,
        /// secret message to write to PNG
        message: String,
    },
    /// Decode message from PNG
    #[clap(arg_required_else_help = true)]
    Decode {
        ///  PNG file path
        path: String,
        /// chunckType: four letters
        chunk_type: String,
    },
    /// Remove Chunk from PNG
    #[clap(arg_required_else_help = true)]
    Remove {
        ///  PNG file path
        path: String,
        /// chunckType: four letters
        chunk_type: String,
    },
    /// Print PNG info
    #[clap(arg_required_else_help = true)]
    Print {
        /// Stuff to add
        path: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    match &args.command {
        Commands::Decode { path, chunk_type } => {
            let bytes = fs::read(path)?;
            let png = Png::try_from(&bytes[..])?;
            let chunk = png.chunk_by_type(chunk_type);
            if let Some(chunk) = chunk {
                println!("message: {}", chunk.data_as_string().unwrap());
            } else {
                println!("No chunk found");
            }
        }
        Commands::Encode {
            path,
            chunk_type,
            message,
        } => {
            let bytes = fs::read(path)?;
            let mut png = Png::try_from(&bytes[..])?;

            let chunk = Chunk::new(
                ChunkType::from_str(chunk_type).unwrap(),
                message.as_bytes().to_vec(),
            );

            png.append_chunk(chunk);

            // save png
            fs::write(path, png.as_bytes())?;
        }
        Commands::Remove { path, chunk_type } => {
            let bytes = fs::read(path)?;
            let mut png = Png::try_from(&bytes[..])?;
            png.remove_chunk(chunk_type)?;

            // save png
            fs::write(path, png.as_bytes())?;
        }
        Commands::Print { path } => {
            let bytes = fs::read(path)?;
            let png = Png::try_from(&bytes[..])?;
            println!("PNG file: {}", path);
            for chunk in png.chunks() {
                println!("{}", chunk);
            }
        }
    }

    Ok(())
}
