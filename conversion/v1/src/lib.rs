use std::fs;

//get list of rules
//rules have their data
//rule has trait apply transformation
#[derive(Clone, Debug)]
pub enum Rule<'a> {
    Replace(&'a str, &'a str),
    EndReplace(&'a str, &'a str),
}

trait Replacement {
    fn apply_replacement(&self, word: &str) -> String;
}

impl Replacement for Rule<'_> {
    fn apply_replacement(&self, word: &str) -> String {
        match self {
            Rule::Replace(left, right) => replace(word, (left, right)),
            Rule::EndReplace(left, right) => end_replace(word, (left, right)),
        }
    }
}

pub fn rule_to_tuple(rule: &str) -> (String, String) {
    let left_right = rule.split(" -> ").collect::<Vec<&str>>();
    (left_right[0].to_string(), left_right[1].to_string())
}

pub fn line_to_rule(line: &str) -> Rule {
    let left_right = line.split(" -> ").collect::<Vec<&str>>();
    let left = left_right[0];
    let right = left_right[1];

    match left.contains("...") {
        true => Rule::EndReplace(left, right),
        false => Rule::Replace(left, right),
    }
}

pub fn get_rules<'a>(content: &'a str) -> Vec<Rule<'a>> {
    let lines: Vec<&str> = content.lines().collect();

    lines
        .iter()
        .map(|line| line_to_rule(&line))
        .collect::<Vec<Rule>>()
}

fn replace(content: &str, tuple: (&str, &str)) -> String {
    let result = content.replace(tuple.0, tuple.1);

    String::from(result)
}

// replaces string if it ends on a certain string
//
// for instance:
// ...dt -> t
// wordt -> wort
fn end_replace(word: &str, tuple: (&str, &str)) -> String {
    // left and right is the inputs of the "rule"
    let left: String = tuple.0.to_string().split("...").take(1).collect();
    let right = tuple.1.to_string();

    let div_len = word.len() - right.len();

    if div_len > 0 {
        let word = String::from(word);

        let begining = &word[..div_len];
        let ending = &word[div_len..];

        match ending {
            left => String::from(begining.to_string() + &right),
            _ => word,
        }
    } else {
        String::from(word)
    }
}

pub fn convert_word_new(word: &str, rules: Vec<Rule>) -> String {
    let rules = rules.clone();

    let word = String::from(word);
    rules
        .iter()
        .map(|rule| rule.apply_replacement(&word))
        .collect::<String>()
}

mod rules;

//todo: ignore ? ! , . -
fn convert_word(word: &str) -> String {
    let mut result = String::from(word);

    result = rules::r1(&result);
    result = rules::r2(&result);
    result = rules::r3(&result);
    result = rules::r4(&result);
    result = rules::r5(&result);
    result = rules::r6(&result);
    result = rules::r7(&result);
    result = rules::r8(&result);
    result = rules::r9(&result);
    result = rules::r10(&result);
    result = rules::r11(&result);
    result = rules::r12(&result);
    result = rules::r13(&result);

    result
}

pub fn convert_string(content: &str) -> String {
    //apply new-read to every word
    let result: String = content
        .split(' ')
        .map(|word| convert_word(word) + " ")
        .collect::<String>();

    //remove last space from the string
    let mut result = result.chars();
    result.next_back();
    result.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    //example for the tests
    pub struct Example {
        input: String,
        output: String,
    }

    impl Example {
        pub fn build(input: &str, output: &str) -> Self {
            Example {
                input: String::from(input),
                output: String::from(output),
            }
        }
    }

    pub fn test_conversion(rule: fn(&str) -> String, example: &Example) -> bool {
        let input = &example.input;
        let output = &example.output;

        let result = &rule(&input) == output;

        match result {
            true => result,
            false => {
                print!("{} , {} -> {}", input, output, rule(&input));
                result
            }
        }
    }

    #[test]
    fn test_convert_string() {
        let examples = vec![
            Example::build("waarschijnlijk", "waarsgynlyk"),
            Example::build("geschiedenis", "gesgiedenis"),
            Example::build("hoogleraar", "hoogleraar"),
            Example::build("vogel", "vogel"),
            Example::build("fiets", "fiets"),
            Example::build("klank", "klaqk"),
            Example::build("eindelijk", "yndelyk"),
            Example::build("oude mensen", "oude mensen"),
            Example::build("blij zijn", "bly zyn"),
            Example::build("cadeau", "kado"),
            Example::build("auto", "outo"),
        ];

        examples
            .iter()
            .for_each(|example| assert!(test_conversion(convert_string, example)));
    }
}
