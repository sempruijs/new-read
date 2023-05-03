use nrc::*;
use std::fs;
use std::process;

fn main() {
    let rules_string = fs::read_to_string("./src/rules.txt").expect("rules.txt not found");
    let rules = get_rules(&rules_string);

    let content = fs::read_to_string("test.md").expect("test.md not found");
    let new_read_content = convert_string(&content, &rules);

    println!("{}", new_read_content);
}
