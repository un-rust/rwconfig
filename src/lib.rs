use log::trace;

pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    trace!("add_two_numbers(a, b) = {}", a + b);
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(1, 2), 3);
    }
}
