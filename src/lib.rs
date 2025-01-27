#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    #[test]
    async fn trivial() {
        assert!(true);
    }
}
