pub use clap::Parser;
pub mod bitvec_set;
pub mod grid_util;
pub mod vm;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: String,
}
