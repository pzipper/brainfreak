use clap::{Args, Parser};

/// The Brainfreak compiler.
#[derive(Parser)]
pub enum Command {
    /// Compiles the provided Brainfreak program into an object file.
    BuildObj(BuildObj),
}

/// Arguments for building an object file.
#[derive(Args)]
pub struct BuildObj {
    /// The path to the Brainfreak program to compile.
    pub input_path: String,

    /// The path to write the created object file to.
    #[arg(short = 'o')]
    pub output_path: String,
}
