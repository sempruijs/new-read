//version 0.0.1
use std::fs;
use v1::convert_word;
use v1::get_rules;

fn main() {
    let rules_content = fs::read_to_string("./src/rules.txt").expect("problem reading rules file");
    let rules = get_rules(&rules_content);

    let old_word = "eigenlijk";
    let new_word = convert_word(old_word, rules);

    println!("{}", new_word);
}
