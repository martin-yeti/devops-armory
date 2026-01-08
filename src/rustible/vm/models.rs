use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
//#[clap(trailing_var_arg = true)]
pub struct Cli {

    #[clap(long, value_delimiter(','))]
    pub host: Vec<String>,

    //#[clap(long, default_value_t = 22)]
    //pub port: u16,

    #[clap(required(true))]
    pub username: String,

    #[clap(required(true))]
    pub private_key: PathBuf,

    #[clap(required(true), value_delimiter(','))]
    pub command: Vec<String>,

}