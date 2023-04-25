use std::fs;
use std::process;
use v1::convert_string;
use v1::get_context;
use v1::get_rules;
use v1::ContextError;

fn main() {
    let context = get_context();
    match context {
        Ok(context) => {
            let rules_content =
                fs::read_to_string("./src/rules.txt").expect("problem reading rules file");
            let rules = get_rules(&rules_content);

            let content = fs::read_to_string(&context.file_path).expect("test.md not found");
            let new_read_content = convert_string(&content, &rules);

            println!("{}", new_read_content);
        }
        Err(context_error) => match context_error {
            ContextError::NoPath => {
                eprintln!("Not enough arguments. Please profide a filepath. For example\n\n    nrc example.txt\n");
                process::exit(1)
            }
            ContextError::NotFound => {
                eprintln!("not a vallid file path");
            }
        },
    }
}
