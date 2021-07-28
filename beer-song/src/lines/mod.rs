mod first_line;
mod second_line;

use first_line::get_first_line;
use second_line::get_second_line;

pub fn get_verse_pair(bottles: u32, total_bottles: u32) -> String {
    format!(
        "{}\n{}\n",
        get_first_line(bottles),
        get_second_line(bottles, total_bottles)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_lines_for_0_bottles() {
        assert_eq!(
            get_verse_pair(0, 99),
            "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        );
    }

    #[test]
    fn should_return_correct_lines_for_1_bottle() {
        assert_eq!(
            get_verse_pair(1, 99),
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"
        );
    }

    #[test]
    fn should_return_correct_lines_for_2_bottles() {
        assert_eq!(
            get_verse_pair(2, 99),
            "2 bottles of beer on the wall, 2 bottles of beer.\n\
            Take one down and pass it around, 1 bottle of beer on the wall.\n"
        );
    }

    #[test]
    fn should_return_correct_lines_for_8_bottles() {
        assert_eq!(
            get_verse_pair(8, 99),
            "8 bottles of beer on the wall, 8 bottles of beer.\n\
            Take one down and pass it around, 7 bottles of beer on the wall.\n"
        );
    }
}
