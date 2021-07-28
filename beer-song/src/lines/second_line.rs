pub fn get_second_line(bottles: u32, total_bottles: u32) -> String {
    match bottles {
        0 => format!(
            "Go to the store and buy some more, {0} bottles of beer on the wall.",
            total_bottles,
        ),
        1 => "Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
        2 => "Take one down and pass it around, 1 bottle of beer on the wall.".to_string(),
        bottles => format!(
            "Take one down and pass it around, {0} bottles of beer on the wall.",
            bottles - 1,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_line_for_0_bottles() {
        assert_eq!(
            get_second_line(0, 99),
            "Go to the store and buy some more, 99 bottles of beer on the wall."
        )
    }

    #[test]
    fn should_return_correct_line_for_0_bottles_and_42_total() {
        assert_eq!(
            get_second_line(0, 42),
            "Go to the store and buy some more, 42 bottles of beer on the wall."
        )
    }

    #[test]
    fn should_return_correct_line_for_1_bottle() {
        assert_eq!(
            get_second_line(1, 99),
            "Take it down and pass it around, no more bottles of beer on the wall."
        )
    }

    #[test]
    fn should_return_correct_line_for_2_bottles() {
        assert_eq!(
            get_second_line(2, 99),
            "Take one down and pass it around, 1 bottle of beer on the wall."
        )
    }

    #[test]
    fn should_return_correct_line_for_42_bottles() {
        assert_eq!(
            get_second_line(42, 99),
            "Take one down and pass it around, 41 bottles of beer on the wall."
        )
    }
}
