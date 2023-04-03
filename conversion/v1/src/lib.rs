pub mod rules {
    use super::replace;

    /// # rule 1
    ///
    /// ch -> sj
    ///
    pub fn r1(content: &str) -> String {
        replace(content, "ch", "sj")
    }

    /// # rule 2
    ///
    /// ij -> y
    ///
    pub fn r2(content: &str) -> String {
        replace(content, "ij", "y")
    }

    /// # rule 3
    ///
    /// ei -> y
    ///
    pub fn r3(content: &str) -> String {
        replace(content, "ei", "y")
    }
}

pub fn replace(content: &str, old: &str, new: &str) -> String {
    let result = content.replace(old, new);

    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    //example for the tests
    struct Example {
        input: String,
        output: String,
    }

    impl Example {
        fn build(input: &str, output: &str) -> Self {
            Example {
                input: String::from(input),
                output: String::from(output),
            }
        }
    }

    fn test_conversion(rule: fn(&str) -> String, example: Example) -> bool {
        let input = example.input;
        let output = example.output;

        rule(&input) == output
    }

    #[test]
    fn r1() {
        let example = Example::build("chocola", "sjocola");

        assert!(test_conversion(rules::r1, example));
    }

    #[test]
    fn r2() {
        let example_1 = Example::build("ijsje", "ysje");
        let example_2 = Example::build("belangrijk", "belangryk");

        assert!(test_conversion(rules::r2, example_1));
        assert!(test_conversion(rules::r2, example_2));
    }

    #[test]
    fn r3() {
        let example_1 = Example::build("eindelijk", "yndelijk");
        let example_2 = Example::build("eisen", "ysen");

        assert!(test_conversion(rules::r3, example_1));
        assert!(test_conversion(rules::r3, example_2));
    }
}
