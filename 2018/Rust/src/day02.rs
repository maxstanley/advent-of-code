use std::ops::AddAssign;

#[derive(Debug)]
#[derive(PartialEq)]
struct RepeatingLetters(u16, u16);

impl AddAssign for RepeatingLetters {
    fn add_assign(&mut self, other: Self) {
        *self = Self (
            self.0 + other.0,
            self.1 + other.1,
        );
    }
}

use std::collections::HashMap;

#[allow(dead_code)]
fn has_repeating_letters(string: String) -> RepeatingLetters {
    let mut map: HashMap<char, u16> = HashMap::new();
    for c in string.chars().enumerate() {
        // If no value present in entry, an initial value of 0 is used, which is then
        // immediately incremented.
        *map.entry(c.1).or_insert(0) += 1;
    }
    let mut repeats = RepeatingLetters (0, 0);
    for v in map.values() {
        if *v == 2 {
            repeats.0 = 1;
        } else if *v == 3 {
            repeats.1 = 1;
        }
    }
    repeats
}

#[allow(dead_code)]
fn checksum(id_vector: Vec<String>) -> u16 {
    let mut counts = RepeatingLetters (0, 0);
    for id in id_vector {
        counts += has_repeating_letters(id)
    }
    counts.0 * counts.1
}

fn one_off(a: &String, b: &String) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut diffs = 0;
    for i in 0..a.len() {
        if a_bytes[i] != b_bytes[i] {
            if diffs == 1 {
                return false;
            } else {
                diffs = 1;
            }
        }
    }
    true
}

fn common_chars(a: &String, b: &String) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut string_builder = String::new();
    for i in 0..a.len() {
        if a_bytes[i] == b_bytes[i] {
            string_builder.push(a_bytes[i].into());
        }
    }
    println!("{}", string_builder);
    string_builder
}

#[allow(dead_code)]
fn one_char_off_common(id_vector: Vec<String>) -> String {
    let mut vec = id_vector.clone();
    // should probably do a reverse if one way doesn't find a solution, as it
    // is likely the first char that is different.
    vec.sort();

    let mut previous = String::from("");
    for id in vec {
        println!("id: {}", id);
        if one_off(&previous, &id) {
            println!("One Off!");
            return common_chars(&previous, &id);
        }
        previous = id;
    }

    String::from("")
}

#[cfg(test)]
mod tests {
    // This imports names from the outer scope.
    use super::*;
    use crate::common;

    #[test]
    fn example_1() {
        assert_eq!(has_repeating_letters(String::from("abcdef")), RepeatingLetters (0, 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(has_repeating_letters(String::from("bababc")), RepeatingLetters (1, 1));
    }

    #[test]
    fn example_3() {
        assert_eq!(has_repeating_letters(String::from("abbcde")), RepeatingLetters (1, 0));
    }

    #[test]
    fn example_4() {
        assert_eq!(has_repeating_letters(String::from("abcccd")), RepeatingLetters (0, 1));
    }

    #[test]
    fn example_5() {
        assert_eq!(has_repeating_letters(String::from("aabcdd")), RepeatingLetters (1, 0));
    }

    #[test]
    fn example_6() {
        assert_eq!(has_repeating_letters(String::from("abcdee")), RepeatingLetters (1, 0));
    }

    #[test]
    fn example_7() {
        assert_eq!(has_repeating_letters(String::from("ababab")), RepeatingLetters (0,1));
    }

    #[test]
    fn example_8() {
        let mut vec = Vec::new();
        vec![
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
        ].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(checksum(vec), 12);
    }

    #[test]
    fn part_1() {
        let vec = common::lines_from_file(String::from("input/day02.txt"));
        assert_eq!(checksum(vec), 6696);
    }

    #[test]
    fn example_9() {
        let mut vec = Vec::new();
        vec![
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "fguij",
            "wvxyz",
        ].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(one_char_off_common(vec), String::from("fgij"));
    }

    #[test]
    fn part_2() {
        let vec = common::lines_from_file(String::from("input/day02.txt"));
        assert_eq!(one_char_off_common(vec), String::from("bvnfawcnyoeyudzrpgslimtkj"));
    }
}
