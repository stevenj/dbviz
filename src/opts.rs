use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
#[structopt(
    version = "1.0",
    author = "yunmikun <yunmikun2@protonmail.com>, Steven Johnson"
)]
pub struct Opts {
    #[structopt(flatten)]
    pub pg_opts: Pg,

    #[structopt(short)]
    pub include_tables: Option<Vec<String>>,

    #[structopt(short)]
    pub exclude_tables: Option<Vec<String>>,

    #[structopt(long)]
    pub title: Option<String>,

    #[structopt(long)]
    pub column_description_wrap: Option<usize>,

    #[structopt(long)]
    pub table_description_wrap: Option<usize>,

    #[structopt(long)]
    pub comments: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    pub template: Option<PathBuf>,

    // Output file, stdout if not present
    // Not currently implemented.
    //#[structopt(parse(from_os_str))]
    //pub output: Option<PathBuf>,
}

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
#[structopt(version = "1.0", author = "yunmikun <yunmikun2@protonmail.com>")]
pub struct Pg {
    #[structopt(short, long, default_value = "localhost")]
    pub hostname: String,
    #[structopt(short, long, default_value = "postgres")]
    pub username: String,
    #[structopt(short, long, default_value = "postgres")]
    pub password: String,
    #[structopt(short, long, default_value = "postgres")]
    pub database: String,
    #[structopt(short, long, default_value = "public")]
    pub schema: String,
}

/// Load CLI Options.
pub fn load() -> Opts {
    Opts::from_args()
}
