use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// File path to the wordlist
    #[clap(short, long)]
    pub wordlist: String,

    /// Hostname to be tested
    #[clap(short, long)]
    pub url: String,

    /// File extensions
    #[clap(short, long, multiple_values=true)]
    pub extension: Vec<String>,

    /// Number of parallel threads
    #[clap(short, long, value_parser, default_value_t = 20)]
    pub threads: u8
}

