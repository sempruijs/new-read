use crate::converter::*;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod converter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file as input to convert
    #[arg(short, long)]
    file_name: String,

    /// name for output file. If not specified, the output will be printed.
    #[arg(short, long)]
    output_name: Option<String>,
}

fn create_file(name: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    let rules_string = fs::read_to_string("./src/rules.txt").expect("rules.txt not found");
    let rules = get_rules(&rules_string);

    let content = fs::read_to_string(&args.file_name).expect("test.md not found");
    let new_read_content = convert_string(&content, &rules);

    match &args.output_name {
        Some(name) => create_file(&name, &new_read_content).unwrap(),
        None => println!("{}", new_read_content),
    }
}
