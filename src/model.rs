use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Params {
    ///Directory to your flutter project
    #[arg(short, long)]
    pub path: PathBuf,

    ///Version Code of your build
    #[arg(short, long, default_value_t = 1)]
    pub version_code: i64,

    ///Version name of you build e.g 1.0.0
    #[arg(short, long,default_value_t = String::from("1.0.0"))]
    pub build_name: String,
}
