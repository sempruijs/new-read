// # how the convert function is made
//
// the convert function exepts a &str and a Vec<Rule>
// a rule defines how a word should be manipulated
// so, a rule is simply a transformation
// the convert_word function maps all rules on the string
// convert word maps convert_word on every word in the input
//
// rules are generated based on the **rules.txt** file
//
// ## the rule syntax used in **rules.txt**
//
// #### ends on
//
// ...[old ending] -> [new ending]
//
// example:
//
// ...dt -> t
//
// apply it to deffernt inputs
//
// wordt -> wort
// eigenlijk -> eigenlijk
// want -> want
// werdt -> wert
//
//
// #### replace
//
// [old part] -> [new part]
//
// example:
//
// ij -> y
//
// implement this rule on different inputs
//
// eigenlijk -> eigenlyk
// overal -> overal
// ijsjes -> ysjes

// rules have assosiated strings of the transformations
// no syntax transformations are applied.
// rule has trait apply transformation
#[derive(Clone, Debug)]
pub enum Rule<'a> {
    Replace(&'a str, &'a str),
    EndReplace(&'a str, &'a str),
}

/// this will make of a line in the **rules.txt** file a Rule
/// it will NOT check for syntex errors, if thir a incorrect line, it will panic
pub fn line_to_rule(line: &str) -> Rule {
    let left_right = line.split(" -> ").collect::<Vec<&str>>();
    let left = left_right[0];
    let right = left_right[1];

    match left.contains("...") {
        true => Rule::EndReplace(left, right),
        false => Rule::Replace(left, right),
    }
}

trait Replacement {
    fn apply_replacement(&self, word: &str) -> String;
}

impl Replacement for Rule<'_> {
    /// this will apply the rule to the word
    fn apply_replacement(&self, word: &str) -> String {
        match self {
            Rule::Replace(left, right) => replace(word, (left, right)),
            Rule::EndReplace(left, right) => end_replace(word, (left, right)),
        }
    }
}

/// this will get the rules based on the 'content' which should be a **rules.txt** type of file
pub fn get_rules<'a>(content: &'a str) -> Vec<Rule<'a>> {
    let lines: Vec<&str> = content.lines().collect();

    lines
        .iter()
        .map(|line| line_to_rule(&line))
        .collect::<Vec<Rule>>()
}

/// this is the transformation logic for a normal replace
///
///[old part] -> [new part]
///
fn replace(content: &str, tuple: (&str, &str)) -> String {
    let result = content.replace(tuple.0, tuple.1);

    String::from(result)
}

// replaces string if it ends on a certain string
//
// for instance:
// ...dt -> t
// wordt -> wort
//
// we will convert "wordt" to "wort"
fn end_replace(word: &str, tuple: (&str, &str)) -> String {
    // left and right is the inputs of the "rule"
    // so:

    // dt
    let left = tuple.0.replace("...", "");

    //t
    let end = tuple.1;

    // 5
    let len = word.len();

    if len > left.len() {
        // 5 - 2 = 3
        let div_len = len - left.len();

        let (part_without_end, old_end) = word.split_at(div_len);

        if old_end == "dt" {
            return String::from(part_without_end) + end;
        }
    }

    String::from(word)
}

// this will apply the transformation of the rules parsed and return a String with the transformations applied
pub fn convert_word(word: &str, rules: &Vec<Rule>) -> String {
    let rules = rules.clone();

    let mut result = String::from(word);

    rules
        .iter()
        .for_each(|rule| result = rule.apply_replacement(&result));

    result
}

/// converts every word parsed in content and apply every tranformation of the rules and returns a String
pub fn convert_string(content: &str, rules: &Vec<Rule>) -> String {
    //apply new-read to every word
    let result: String = content
        .split(' ')
        .map(|word| convert_word(word, rules) + " ")
        .collect::<String>();

    //remove last space from the string
    let mut result = result.chars();
    result.next_back();
    result.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    pub fn test_end_replace() {
        let rule = Rule::EndReplace("...dt", "t");
        let input = "wordt";
        let expected_outupt = String::from("wort");

        let output = rule.apply_replacement(input);

        assert_eq!(output, expected_outupt);
    }

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

    fn test_conversion(example: &Example) {
        let rules_content =
            fs::read_to_string("./src/rules.txt").expect("problem reading rules file");
        let rules = get_rules(&rules_content);

        let input = &example.input;
        let output = convert_string(&input, &rules);

        let expected_output = &example.output;
        assert_eq!(&output, expected_output);
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
            .for_each(|example| test_conversion(&example));
    }
}
