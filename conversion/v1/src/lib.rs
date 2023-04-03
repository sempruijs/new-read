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
}

pub fn replace(content: &str, old: &str, new: &str) -> String {
    let result = content.replace(old, new);

    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r1() {
        let example_1 = "chocola";
        let result = rules::r1(&example_1);

        assert_eq!(result, "sjocola");
    }

    #[test]
    fn r2() {
        let example_1 = "ijsje";
        let example_2 = "belangrijk";

        let result_1 = rules::r2(&example_1);
        let result_2 = rules::r2(&example_2);

        assert_eq!(result_1, "ysje");
        assert_eq!(result_2, "belangryk");
    }
}
