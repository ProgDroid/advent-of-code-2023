use log::debug;

use std::{env::args, fs::read_to_string, str::FromStr};

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

const DELIMITER_GAME: &str = ":";
const DELIMITER_SET: &str = ";";
const DELIMITER_CUBE: &str = ",";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl Colour {
    const fn max(self) -> u8 {
        match self {
            Self::Red => MAX_RED,
            Self::Green => MAX_GREEN,
            Self::Blue => MAX_BLUE,
        }
    }
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

#[derive(Clone, Copy, Debug)]
struct ColouredCube {
    amount: u8,
    colour: Colour,
}

impl ColouredCube {
    const fn is_valid(self) -> bool {
        self.amount <= self.colour.max()
    }
}

#[derive(Debug)]
struct CubeSet {
    cubes: Vec<ColouredCube>,
}

impl CubeSet {
    fn is_valid(&self) -> bool {
        self.cubes.iter().filter(|cube| !cube.is_valid()).count() == 0
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.cube_sets.iter().filter(|set| !set.is_valid()).count() == 0
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

fn calculate_game_id_sum<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    lines
        .filter_map(parse_game)
        .filter(Game::is_possible)
        .map(|game| game.id)
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
                Ok(lines) => println!("Answer: {}", calculate_game_id_sum(lines.lines())),
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

        let result = calculate_game_id_sum(lines);

        assert_eq!(result, 8);
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
    fn test_cube_is_valid() {
        let red_cube_valid = ColouredCube {
            amount: 12,
            colour: Colour::Red,
        };

        let red_cube_invalid = ColouredCube {
            amount: 13,
            colour: Colour::Red,
        };

        let green_cube_valid = ColouredCube {
            amount: 13,
            colour: Colour::Green,
        };

        let green_cube_invalid = ColouredCube {
            amount: 14,
            colour: Colour::Green,
        };

        let blue_cube_valid = ColouredCube {
            amount: 14,
            colour: Colour::Blue,
        };

        let blue_cube_invalid = ColouredCube {
            amount: 15,
            colour: Colour::Blue,
        };

        assert!(red_cube_valid.is_valid());
        assert!(!red_cube_invalid.is_valid());
        assert!(green_cube_valid.is_valid());
        assert!(!green_cube_invalid.is_valid());
        assert!(blue_cube_valid.is_valid());
        assert!(!blue_cube_invalid.is_valid());
    }

    #[test]
    fn test_cube_set_is_valid() {
        let cube_set_valid = CubeSet {
            cubes: vec![
                ColouredCube {
                    amount: 12,
                    colour: Colour::Red,
                },
                ColouredCube {
                    amount: 1,
                    colour: Colour::Red,
                },
                ColouredCube {
                    amount: 13,
                    colour: Colour::Green,
                },
            ],
        };

        let cube_set_invalid = CubeSet {
            cubes: vec![
                ColouredCube {
                    amount: 12,
                    colour: Colour::Red,
                },
                ColouredCube {
                    amount: 13,
                    colour: Colour::Red,
                },
                ColouredCube {
                    amount: 13,
                    colour: Colour::Green,
                },
            ],
        };

        assert!(cube_set_valid.is_valid());
        assert!(!cube_set_invalid.is_valid());
    }

    #[test]
    fn test_game_is_possible() {
        let game_possible = Game {
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

        let game_impossible = Game {
            id: 1,
            cube_sets: vec![
                CubeSet {
                    cubes: vec![
                        ColouredCube {
                            amount: 3,
                            colour: Colour::Blue,
                        },
                        ColouredCube {
                            amount: 20,
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

        assert!(game_possible.is_possible());
        assert!(!game_impossible.is_possible());
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
}
