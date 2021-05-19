use n2k_codegen::N2kCodeGenOpts;
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(long)]
    /// Path to the CanBoat project pgns.xml, or derivative of
    pub pgns_xml: String,
    #[structopt(short = "p", long = "pgn")]
    /// List of PGNs to generate decoders for
    pub pgns: Vec<u32>,
    #[structopt(short, long)]
    /// Path to the output directory of the crate or module
    pub output: PathBuf,
    #[structopt(short, long)]
    /// Whether to output a complete crate, and if so, the name of it. Will generate a module otherwise.
    pub crate_name: Option<String>,
}

pub fn main() {
    env_logger::init();
    let opts = Opts::from_args();

    let args = N2kCodeGenOpts {
        pgns_xml: opts.pgns_xml,
        pgns: opts.pgns.iter().cloned().collect(),
        output: opts.output,
        generate_crate: opts.crate_name,
    };

    n2k_codegen::codegen(args);
}
