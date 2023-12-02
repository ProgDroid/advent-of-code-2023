use log::debug;

use std::{env::args, fs::read_to_string, str::FromStr};

const DELIMITER_GAME: &str = ":";
const DELIMITER_SET: &str = ";";
const DELIMITER_CUBE: &str = ",";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl FromStr for Colour {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("Could not convert to colour"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ColouredCube {
    amount: u8,
    colour: Colour,
}

#[derive(Debug)]
struct CubeSet {
    cubes: Vec<ColouredCube>,
}

impl CubeSet {
    fn get_power(self) -> u32 {
        self.cubes
            .iter()
            .map(|cube| Into::<u32>::into(cube.amount))
            .product()
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn get_minimum_cube_set(self) -> Option<CubeSet> {
        let mut cubes_red: Vec<&ColouredCube> = self
            .cube_sets
            .iter()
            .flat_map(|set| set.cubes.iter().filter(|cube| cube.colour == Colour::Red))
            .collect();

        cubes_red.sort();

        let mut cubes_green: Vec<&ColouredCube> = self
            .cube_sets
            .iter()
            .flat_map(|set| set.cubes.iter().filter(|cube| cube.colour == Colour::Green))
            .collect();

        cubes_green.sort();

        let mut cubes_blue: Vec<&ColouredCube> = self
            .cube_sets
            .iter()
            .flat_map(|set| set.cubes.iter().filter(|cube| cube.colour == Colour::Blue))
            .collect();

        cubes_blue.sort();

        if cubes_red.is_empty() && cubes_green.is_empty() && cubes_blue.is_empty() {
            return None;
        }

        let mut cubes: Vec<ColouredCube> = Vec::default();

        if let Some(cube_red) = cubes_red.last() {
            cubes.push(**cube_red);
        }

        if let Some(cube_green) = cubes_green.last() {
            cubes.push(**cube_green);
        }

        if let Some(cube_blue) = cubes_blue.last() {
            cubes.push(**cube_blue);
        }

        Some(CubeSet { cubes })
    }
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn parse_cube(cube: &str) -> Option<ColouredCube> {
    let tokens: Vec<&str> = cube.split_whitespace().collect();

    if let Some(amount_str) = tokens.first() {
        if let Some(colour_str) = tokens.get(1) {
            let amount = amount_str.parse::<u8>().ok();
            let colour = Colour::from_str(colour_str).ok();

            if amount.is_none() || colour.is_none() {
                log("Could not parse cube, invalid amount or colour");
                return None;
            }

            return Some(ColouredCube {
                amount: amount.unwrap(),
                colour: colour.unwrap(),
            });
        }
    }

    log("Could not parse cube");

    None
}

fn parse_set(set: &str) -> Option<CubeSet> {
    log("Parsing set");

    let cubes: Vec<ColouredCube> = set
        .trim()
        .split(DELIMITER_CUBE)
        .filter_map(parse_cube)
        .collect();

    if cubes.is_empty() {
        None
    } else {
        Some(CubeSet { cubes })
    }
}

fn parse_sets_string(sets: &str) -> Option<Vec<CubeSet>> {
    log("Parsing sets");

    let vec: Vec<CubeSet> = sets.split(DELIMITER_SET).filter_map(parse_set).collect();

    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

fn parse_game_string(game: &str) -> Option<u32> {
    log("Parsing game");

    if let Some((_, string_id)) = game.trim().split_once(' ') {
        return string_id.parse::<u32>().ok();
    }

    None
}

fn parse_game(line: &str) -> Option<Game> {
    log(format!("Parsing {line}").as_str());

    if let Some((game_string, sets_string)) = line.split_once(DELIMITER_GAME) {
        match parse_game_string(game_string) {
            Some(game_id) => match parse_sets_string(sets_string) {
                Some(game_sets) => {
                    return Some(Game {
                        id: game_id,
                        cube_sets: game_sets,
                    });
                }
                None => log("Could not parse game sets"),
            },
            None => log("Could not parse game ID"),
        }
    }

    None
}

fn calculate_minimum_required_set_power_sum<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    lines
        .filter_map(parse_game)
        .filter_map(Game::get_minimum_cube_set)
        .map(CubeSet::get_power)
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let args: Vec<String> = args().collect();

    args.get(1).map_or_else(
        || {
            eprintln!("No input file path provided");
        },
        |file_name| {
            match read_to_string(file_name) {
                Ok(lines) => println!(
                    "Answer: {}",
                    calculate_minimum_required_set_power_sum(lines.lines())
                ),
                Err(e) => eprintln!("Could not load input file {file_name}: {e}"),
            };
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines();

        let result = calculate_minimum_required_set_power_sum(lines);

        assert_eq!(result, 2286);
    }

    #[test]
    fn test_parse_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let expected = Game {
            id: 1,
            cube_sets: vec![
                CubeSet {
                    cubes: vec![
                        ColouredCube {
                            amount: 3,
                            colour: Colour::Blue,
                        },
                        ColouredCube {
                            amount: 4,
                            colour: Colour::Red,
                        },
                    ],
                },
                CubeSet {
                    cubes: vec![
                        ColouredCube {
                            amount: 1,
                            colour: Colour::Red,
                        },
                        ColouredCube {
                            amount: 2,
                            colour: Colour::Green,
                        },
                        ColouredCube {
                            amount: 6,
                            colour: Colour::Blue,
                        },
                    ],
                },
                CubeSet {
                    cubes: vec![ColouredCube {
                        amount: 2,
                        colour: Colour::Green,
                    }],
                },
            ],
        };

        let game = parse_game(line);

        assert!(game.is_some());

        let game = game.unwrap();

        assert_eq!(expected.id, game.id);
        assert_eq!(expected.cube_sets.len(), game.cube_sets.len());
    }

    #[test]
    fn test_parse_game_string() {
        let game = "Game 1";

        let game_id = parse_game_string(game);

        assert_eq!(Some(1), game_id);

        let game = "Game 10";

        let game_id = parse_game_string(game);

        assert_eq!(Some(10), game_id);
    }

    #[test]
    fn test_parse_sets_string() {
        let sets = " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result = parse_sets_string(sets);

        let expected = vec![
            CubeSet {
                cubes: vec![
                    ColouredCube {
                        amount: 3,
                        colour: Colour::Blue,
                    },
                    ColouredCube {
                        amount: 4,
                        colour: Colour::Red,
                    },
                ],
            },
            CubeSet {
                cubes: vec![
                    ColouredCube {
                        amount: 1,
                        colour: Colour::Red,
                    },
                    ColouredCube {
                        amount: 2,
                        colour: Colour::Green,
                    },
                    ColouredCube {
                        amount: 6,
                        colour: Colour::Blue,
                    },
                ],
            },
            CubeSet {
                cubes: vec![ColouredCube {
                    amount: 2,
                    colour: Colour::Green,
                }],
            },
        ];

        assert!(result.is_some());

        assert_eq!(result.unwrap().len(), expected.len());
    }

    #[test]
    fn test_parse_set() {
        let set = "3 blue, 4 red";

        let result = parse_set(set);

        assert!(result.is_some());

        assert_eq!(result.unwrap().cubes.len(), 2);
    }

    #[test]
    fn test_parse_cube() {
        let cube = "3 blue";

        let result = parse_cube(cube);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result.amount, 3);
        assert_eq!(result.colour, Colour::Blue);
    }

    #[test]
    fn test_get_minimum_cube_set() {
        let cubes_1: Vec<ColouredCube> = vec![
            ColouredCube {
                amount: 1,
                colour: Colour::Red,
            },
            ColouredCube {
                amount: 10,
                colour: Colour::Green,
            },
            ColouredCube {
                amount: 4,
                colour: Colour::Blue,
            },
        ];

        let cube_set_1 = CubeSet { cubes: cubes_1 };

        let cubes_2: Vec<ColouredCube> = vec![
            ColouredCube {
                amount: 11,
                colour: Colour::Red,
            },
            ColouredCube {
                amount: 2,
                colour: Colour::Blue,
            },
        ];

        let cube_set_2 = CubeSet { cubes: cubes_2 };

        let game = Game {
            id: 1,
            cube_sets: vec![cube_set_1, cube_set_2],
        };

        let result = game.get_minimum_cube_set();

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result.cubes.len(), 3);
    }

    #[test]
    fn test_cube_set_get_power() {
        let cubes: Vec<ColouredCube> = vec![
            ColouredCube {
                amount: 1,
                colour: Colour::Red,
            },
            ColouredCube {
                amount: 10,
                colour: Colour::Green,
            },
            ColouredCube {
                amount: 4,
                colour: Colour::Blue,
            },
        ];

        let cube_set = CubeSet { cubes };

        let result = cube_set.get_power();

        assert_eq!(result, 40);
    }
}
