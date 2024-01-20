pub fn thing() -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(thing(), 42);
    }
}
