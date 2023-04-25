//version 0.0.1
use std::fs;
use v1::convert_string;
use v1::get_context;
use v1::get_rules;

fn main() {
    let context = get_context();

    let rules_content = fs::read_to_string("./src/rules.txt").expect("problem reading rules file");
    let rules = get_rules(&rules_content);

    let content = fs::read_to_string(&context.file_path).expect("test.md not found");
    let new_read_content = convert_string(&content, &rules);

    println!("{}", new_read_content);
}
