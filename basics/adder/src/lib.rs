pub fn greet(name: &str) -> String {
    format!("hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_says_name() {
        let res = greet("Ankan");
        assert!(
            res.contains("Ankan"),
            "Greeting did not contain name, it contained {result}"
        );
    }
}
