mod test_utils;
mod lexer;
mod parser;

#[cfg(test)]
pub mod test {

    #[test]
    pub fn sanity_check() {
        assert_eq!(1, 1);
    }
}