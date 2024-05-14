use std::path::PathBuf;

use structopt::StructOpt;


#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(short = "r", long = "result", parse(from_os_str))]
    result_file: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>
}
pub fn main() {
    println!("{:#?}", Args::from_args());
}