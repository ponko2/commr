use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file 1")]
    file1: String,

    #[arg(help = "Input file 2")]
    file2: String,

    #[arg(short = '1', help = "Suppress printing of column 1")]
    suppress_col1: bool,

    #[arg(short = '2', help = "Suppress printing of column 2")]
    suppress_col2: bool,

    #[arg(short = '3', help = "Suppress printing of column 3")]
    suppress_col3: bool,

    #[arg(short, help = "Case-insensitive comparison of lines")]
    insensitive: bool,

    #[arg(
        short,
        long = "output-delimiter",
        value_name = "DELIM",
        help = "Output delimiter",
        default_value = "\t"
    )]
    delimiter: String,
}

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

pub fn get_args() -> Result<Config> {
    let args = Args::parse();
    Ok(Config {
        file1: args.file1,
        file2: args.file2,
        show_col1: !args.suppress_col1,
        show_col2: !args.suppress_col2,
        show_col3: !args.suppress_col3,
        insensitive: args.insensitive,
        delimiter: args.delimiter,
    })
}

pub fn run(config: Config) -> Result<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        bail!("Both input files cannot be STDIN (\"-\")");
    }

    let _file1 = open(file1)?;
    let _file2 = open(file2)?;
    println!("Opened {file1} and {file2}");

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    Ok(match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(
            File::open(filename).map_err(|err| anyhow!("{filename}: {err}"))?,
        )),
    })
}
