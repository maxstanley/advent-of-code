use crate::error::Error;

use std::str::FromStr;

struct Submarine {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Default for Submarine {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Submarine {
    fn new() -> Self {
        Default::default()
    }

    // part01.
    pub fn modify_position(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(x) => self.horizontal += x,
            Direction::Down(x) => self.depth += x,
            Direction::Up(x) => self.depth -= x,
        }
    }

    // part02.
    pub fn modify_aim(&mut self, aim: &Direction) {
        match aim {
            Direction::Forward(x) => {
                self.horizontal += x;
                self.depth += x * self.aim;
            }
            Direction::Down(x) => self.aim += x,
            Direction::Up(x) => self.aim -= x,
        }
    }
}

enum Direction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, magnitude) = s.split_once(" ").ok_or(Error::ParseError)?;

        let magnitude = magnitude.parse::<i64>()?;

        let direction = match axis {
            "forward" => Self::Forward(magnitude),
            "up" => Self::Up(magnitude),
            "down" => Self::Down(magnitude),
            _ => {
                return Err(Error::ParseError);
            }
        };

        Ok(direction)
    }
}

fn get_directions<T>(lines: &[T]) -> Result<Vec<Direction>, Error>
where
    T: AsRef<str>,
{
    let list = lines
        .into_iter()
        // Parse each line into a Direction.
        // If the parse is unsuccessful, map the error to the custom error type.
        .map(|l| l.as_ref().parse::<Direction>().map_err(Into::into))
        // Collect the results together, and return the error if one occured.
        .collect::<Result<_, Error>>()?;

    // Return the list of depths.
    Ok(list)
}

pub fn part01<T>(lines: &[T]) -> Result<i64, Error>
where
    T: AsRef<str>,
{
    let directions = get_directions(lines)?;

    let mut submarine = Submarine::new();
    directions.iter().for_each(|d| submarine.modify_position(d));

    Ok(submarine.depth * submarine.horizontal)
}

pub fn part02<T>(lines: &[T]) -> Result<i64, Error>
where
    T: AsRef<str>,
{
    let directions = get_directions(lines)?;

    let mut submarine = Submarine::new();
    directions.iter().for_each(|d| submarine.modify_aim(d));

    Ok(submarine.depth * submarine.horizontal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const PART01_EXAMPLE: i64 = 150;
    const PART01: i64 = 1693300;
    const PART02_EXAMPLE: i64 = 900;
    const PART02: i64 = 1857958050;

    #[test]
    fn day02_part01_example() {
        let lines = common::read_lines_from_file("../input/day02_example.txt").unwrap();

        assert_eq!(part01(&lines).unwrap(), PART01_EXAMPLE);
    }

    #[test]
    fn day02_part01() {
        let lines = common::read_lines_from_file("../input/day02.txt").unwrap();

        assert_eq!(part01(&lines).unwrap(), PART01);
    }

    #[test]
    fn day02_part02_example() {
        let lines = common::read_lines_from_file("../input/day02_example.txt").unwrap();

        assert_eq!(part02(&lines).unwrap(), PART02_EXAMPLE);
    }

    #[test]
    fn day02_part02() {
        let lines = common::read_lines_from_file("../input/day02.txt").unwrap();

        assert_eq!(part02(&lines).unwrap(), PART02);
    }
}
