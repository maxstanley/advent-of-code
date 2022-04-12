use crate::error::Error;

fn get_depth_list<T>(lines: &[T]) -> Result<Vec<u64>, Error>
where
    T: AsRef<str>,
{
    let list: Vec<u64> = lines
        .into_iter()
        // Parse each line into a u64.
        // If the parse is unsuccessful, map the error to the custom error type.
        .map(|l| l.as_ref().parse::<u64>().map_err(Into::into))
        // Collect the results together, and return the error if one occured.
        .collect::<Result<_, Error>>()?;

    // Return the list of depths.
    Ok(list)
}

pub fn part01<T>(lines: &[T]) -> Result<usize, Error>
where
    T: AsRef<str>,
{
    let list = get_depth_list(lines)?;

    let count = list
        .iter()
        // Zip the list onto itself with an offset of 1, allowing comparison.
        .zip(list.iter().skip(1))
        // Get the values where b is greater than a.
        .filter(|(a, b)| a < b)
        // Count the number of instances.
        .count();

    Ok(count)
}

pub fn part02<T>(lines: &[T]) -> Result<usize, Error>
where
    T: AsRef<str>,
{
    let list = get_depth_list(lines)?;

    let count = list
        // Group the list into windows of 3.
        .windows(3)
        // Zip the list with itself with an offset of 1.
        .zip(list.windows(3).skip(1))
        // Do the sum over the whole window, and perform the comparison.
        .filter(|(a, b)| a.iter().sum::<u64>() < b.iter().sum())
        // Count the number of instances.
        .count();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const PART01_EXAMPLE: usize = 7;
    const PART01: usize = 1527;
    const PART02_EXAMPLE: usize = 5;
    const PART02: usize = 1575;

    #[test]
    fn day01_part01_example() {
        let lines = common::read_lines_from_file("../input/day01_example.txt").unwrap();

        assert_eq!(part01(&lines).unwrap(), PART01_EXAMPLE);
    }

    #[test]
    fn day01_part01() {
        let lines = common::read_lines_from_file("../input/day01.txt").unwrap();

        assert_eq!(part01(&lines).unwrap(), PART01);
    }

    #[test]
    fn day01_part02_example() {
        let lines = common::read_lines_from_file("../input/day01_example.txt").unwrap();

        assert_eq!(part02(&lines).unwrap(), PART02_EXAMPLE);
    }

    #[test]
    fn day01_part02() {
        let lines = common::read_lines_from_file("../input/day01.txt").unwrap();

        assert_eq!(part02(&lines).unwrap(), PART02);
    }
}
