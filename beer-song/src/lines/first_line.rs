pub fn get_first_line(bottles: u32) -> String {
    match bottles {
        0 => "No more bottles of beer on the wall, no more bottles of beer.".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.".to_string(),
        bottles => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.",
            bottles
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_line_for_0_bottles() {
        assert_eq!(
            get_first_line(0),
            "No more bottles of beer on the wall, no more bottles of beer."
        )
    }

    #[test]
    fn should_return_correct_line_for_1_bottle() {
        assert_eq!(
            get_first_line(1),
            "1 bottle of beer on the wall, 1 bottle of beer."
        )
    }

    #[test]
    fn should_return_correct_line_for_42_bottles() {
        assert_eq!(
            get_first_line(42),
            "42 bottles of beer on the wall, 42 bottles of beer."
        )
    }
}
