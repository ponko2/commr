use crate::Column::*;
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::{
    cmp::Ordering::*,
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
enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
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

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().map_while(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().map_while(Result::ok).map(case);

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(value) => {
                if config.show_col1 {
                    columns.push(value);
                }
            }
            Col2(value) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(value);
                }
            }
            Col3(value) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(value);
                }
            }
        }
        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(value1), Some(value2)) => match value1.cmp(value2) {
                Equal => {
                    print(Col3(value1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(value1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Col2(value2));
                    line2 = lines2.next();
                }
            },
            (Some(value1), None) => {
                print(Col1(value1));
                line1 = lines1.next();
            }
            (None, Some(value2)) => {
                print(Col2(value2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

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
