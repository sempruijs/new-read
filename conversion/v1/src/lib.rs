pub mod rules {
    use super::replace;

    /// # rule 1
    ///
    /// ch -> sj
    ///
    pub fn r1(content: &str) -> String {
        replace(content, "ch", "sj")
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
    fn it_test() {
        let ipt = "chocola";
        let result = rules::r1(&ipt);

        assert_eq!(result, "sjocola");
    }
}
