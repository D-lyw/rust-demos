use clap::{value_parser, Parser};

#[derive(Parser, Debug)]
pub struct PassGenerateOpts {
    #[arg(short, long, default_value_t = 16, value_parser = value_parser!(u8).range(6..))]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
}
