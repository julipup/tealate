pub fn configuration() -> String {
    "configuration".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(configuration(), "configuration".to_string());
    }
}
