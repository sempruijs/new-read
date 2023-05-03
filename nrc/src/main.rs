use clap::Parser;
use nrc::*;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file as input to convert
    #[arg(short, long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();

    let rules_string = fs::read_to_string("./src/rules.txt").expect("rules.txt not found");
    let rules = get_rules(&rules_string);

    let content = fs::read_to_string(&args.file_name).expect("test.md not found");
    let new_read_content = convert_string(&content, &rules);

    println!("{}", new_read_content);
}
