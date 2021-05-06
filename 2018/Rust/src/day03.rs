use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let attributes = s.split(" ").collect::<Vec<&str>>();

        let id = attributes[0].strip_prefix("#").unwrap();

        let mut coordinates = attributes[2].split(",");
        let x = coordinates.next().unwrap_or("0");
        let y = coordinates.next().unwrap_or("0").strip_suffix(":").unwrap();

        let mut dimensions = attributes[3].split("x");
        let width = dimensions.next().unwrap_or("0");
        let height = dimensions.next().unwrap_or("0");

        Ok(Claim{
            id: id.parse()?,
            x: x.parse()?,
            y: y.parse()?,
            width: width.parse()?,
            height: height.parse()?,
        })
    }
}

#[test]
fn parse_claim() {
    assert_eq!("#123 @ 3,2: 5x4".parse::<Claim>().unwrap(), Claim{
        id: 123,
        x: 3,
        y: 2,
        width: 5,
        height: 4,
    });
    assert_eq!("#1 @ 1,3: 4x4".parse::<Claim>().unwrap(), Claim{
        id: 1,
        x: 1,
        y: 3,
        width: 4,
        height: 4,
    });
    assert_eq!("#2 @ 3,1: 4x4".parse::<Claim>().unwrap(), Claim{
        id: 2,
        x: 3,
        y: 1,
        width: 4,
        height: 4,
    });
    assert_eq!("#3 @ 5,5: 2x2".parse::<Claim>().unwrap(), Claim{
        id: 3,
        x: 5,
        y: 5,
        width: 2,
        height: 2,
    });
}

type FabricMap = HashMap<(u32, u32), u32>;

#[allow(dead_code)]
fn parse_claims(claim_string_list: Vec<String>) -> Result<Vec<Claim>, ParseIntError> {
    let mut claims: Vec<Claim> = Vec::new();
    for claim_string in claim_string_list {
        let claim: Claim = claim_string.parse()?;
        claims.push(claim);
    }

    Ok(claims)
}

#[allow(dead_code)]
fn create_fabric_map(claims: &Vec<Claim>) -> FabricMap {
    let mut fabric_map = FabricMap::new();
    
    for claim in claims {
        for x in 0..claim.width{
            for y in 0..claim.height{
                *fabric_map.entry((claim.x + x, claim.y + y)).or_default() += 1;
            }
        }
    }

    fabric_map
}

#[allow(dead_code)]
fn count_overlap(fabric_map: &FabricMap) -> usize {
    fabric_map.values().filter(|&&count| count > 1).count()
}

#[allow(dead_code)]
fn no_overlaps(claims: &Vec<Claim>, fabric_map: &FabricMap) -> u32 {
    let mut id: u32 = 0;
    'claim: for claim in claims {
        for x in 0..claim.width{
            for y in 0..claim.height{
                if *fabric_map.get(&(claim.x + x, claim.y+y)).unwrap() != 1 {
                    continue 'claim;
                }
            }
        }
        id = claim.id;
    }
    id
}

#[test]
fn part_1_and_2() {
    let vec = crate::common::lines_from_file(String::from("input/day03.txt"));
    let claims = parse_claims(vec).unwrap();
    let fabric_map = create_fabric_map(&claims);
    assert_eq!(count_overlap(&fabric_map), 103806);
    assert_eq!(no_overlaps(&claims, &fabric_map), 625);
}
