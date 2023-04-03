pub mod rules {
    /// # rule 1
    ///
    /// ch -> sj
    ///
    pub fn r1(content: String) -> String {
        let result = content.replace("ch", "sj");

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_test() {
        let ipt = "chocola".to_string();
        let result = rules::r1(ipt);

        assert_eq!(result, "sjocola");
    }
}
