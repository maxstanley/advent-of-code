fn total_frequency_change(frequency_change_list: Vec<&str>) -> i8 {
    0
}

#[cfg(test)]
mod tests {
    // This imports names from the outer scope.
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(total_frequency_change(vec!["+1", "-2", "+3", "+1"]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(total_frequency_change(vec!["+1", "+1", "+1"]), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(total_frequency_change(vec!["+1", "+1", "-2"]), 0);
    }

    #[test]
    fn example_4() {
        assert_eq!(total_frequency_change(vec!["-1", "-2", "-3"]), -6);
    }
}
