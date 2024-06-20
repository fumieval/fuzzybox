use std::path;

use clap::*;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,

    #[clap(short, long, env = "OPENAI_API_KEY")]
    pub openai_api_key: String,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Ask {
        /// log format
        #[clap()]
        question: Option<String>,

        #[clap(long = "json")]
        json: bool,
    },
    Uniq {
        /// log format
        #[clap(default_value = "0.9", long = "threshold", short = 't')]
        threshold: f32,

        #[clap(long = "dot")]
        dot: Option<path::PathBuf>,
    },
}
