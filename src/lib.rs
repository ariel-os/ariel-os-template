#![no_main]
#![no_std]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    #[test]
    async fn trivial() {
        assert!(true);
    }
}
