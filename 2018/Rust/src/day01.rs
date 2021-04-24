#[allow(dead_code)]
fn total_frequency_change(frequency_change_list: Vec<String>) -> Result<i32, std::num::ParseIntError> {
    let mut frequency: i32 = 0;
    for item in frequency_change_list {
        if item.eq("") {
            continue;
        }
        frequency += item.parse::<i32>()?;
    }

    Ok(frequency)
}

use hashbrown::HashSet;

#[allow(dead_code)]
fn first_duplicate_frequency(frequency_change_list: Vec<String>) -> Result<i32, std::num::ParseIntError> {
    let mut frequency: i32 = 0;
    let mut frequency_occurences = HashSet::<i32>::default();
    frequency_occurences.insert(frequency);

    loop {
        for item in &frequency_change_list {
            if item.eq("") {
                continue;
            }
            frequency += item.parse::<i32>()?;
            if !frequency_occurences.insert(frequency) {
                return Ok(frequency)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // This imports names from the outer scope.
    use super::*;
    use crate::common;

    #[test]
    fn example_1() {
        let mut vec: Vec<String> = Vec::new();
        vec!["+1", "-2", "+3", "+1"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(total_frequency_change(vec).unwrap(), 3);
    }

    #[test]
    fn example_2() {
        let mut vec: Vec<String> = Vec::new();
        vec!["+1", "+1", "+1"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(total_frequency_change(vec).unwrap(), 3);
    }

    #[test]
    fn example_3() {
        let mut vec: Vec<String> = Vec::new();
        vec!["+1", "+1", "-2"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(total_frequency_change(vec).unwrap(), 0);
    }

    #[test]
    fn example_4() {
        let mut vec: Vec<String> = Vec::new();
        vec!["-1", "-2", "-3"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(total_frequency_change(vec).unwrap(), -6);
    }

    #[test]
    fn part_1() {
        let vec = common::lines_from_file(String::from("input/day01.txt"));
        assert_eq!(total_frequency_change(vec).unwrap(), 442);
    }

    #[test]
    fn example_5() {
        let mut vec: Vec<String> = Vec::new();
        vec!["+1", "-1"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(first_duplicate_frequency(vec).unwrap(), 0);
    }

    #[test]
    fn example_6() {
        let mut vec: Vec<String> = Vec::new();
        vec!["+3", "+3", "+4", "-2", "-4"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(first_duplicate_frequency(vec).unwrap(), 10);
    }

    #[test]
    fn example_7() {
        let mut vec: Vec<String> = Vec::new();
        vec!["-6", "+3", "+8", "+5", "-6"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(first_duplicate_frequency(vec).unwrap(), 5);
    }

    #[test]
    fn example_8() {
        let mut vec: Vec<String> = Vec::new();
        vec!["+7", "+7", "-2", "-7", "-4"].iter().for_each(|&f| vec.push(f.to_string()));
        assert_eq!(first_duplicate_frequency(vec).unwrap(), 14);
    }

    #[test]
    fn part_2() {
        let vec = common::lines_from_file(String::from("input/day01.txt"));
        assert_eq!(first_duplicate_frequency(vec).unwrap(), 59908);
    }
}
