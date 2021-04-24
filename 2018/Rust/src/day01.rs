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
}
