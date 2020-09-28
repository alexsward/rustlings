// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

pub fn get_number() -> u32 {
    44
}

#[cfg(test)]
mod tests {
    use super::get_number;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(get_number(), 44);
    }
}
