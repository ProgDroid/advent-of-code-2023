use log::debug;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(PartialEq, Debug)]
enum TileType {
    PipeVertical,
    PipeHorizontal,
    BendNe,
    BendNw,
    BendSw,
    BendSe,
    Ground,
    StartingPosition,
}

impl TileType {
    const fn from_char(c: char) -> Self {
        match c {
            '|' => Self::PipeVertical,
            '-' => Self::PipeHorizontal,
            'L' => Self::BendNe,
            'J' => Self::BendNw,
            '7' => Self::BendSw,
            'F' => Self::BendSe,
            'S' => Self::StartingPosition,
            _ => Self::Ground, // '.'
        }
    }

    fn has_north(&self) -> bool {
        [Self::PipeVertical, Self::BendNe, Self::BendNw].contains(self)
    }

    fn has_south(&self) -> bool {
        [Self::PipeVertical, Self::BendSe, Self::BendSw].contains(self)
    }

    fn has_east(&self) -> bool {
        [Self::PipeHorizontal, Self::BendNe, Self::BendSe].contains(self)
    }

    fn has_west(&self) -> bool {
        [Self::PipeHorizontal, Self::BendNw, Self::BendSw].contains(self)
    }

    const fn next(&self, entry: &Direction) -> Option<Direction> {
        match self {
            Self::PipeVertical => match entry {
                Direction::South => Some(Direction::North),
                Direction::North => Some(Direction::South),
                _ => None,
            },
            Self::PipeHorizontal => match entry {
                Direction::East => Some(Direction::West),
                Direction::West => Some(Direction::East),
                _ => None,
            },
            Self::BendNe => match entry {
                Direction::North => Some(Direction::East),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Self::BendNw => match entry {
                Direction::North => Some(Direction::West),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Self::BendSe => match entry {
                Direction::South => Some(Direction::East),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            Self::BendSw => match entry {
                Direction::South => Some(Direction::West),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            _ => None,
        }
    }

    const fn initial_direction(&self) -> Direction {
        match self {
            Self::PipeVertical | Self::BendSe | Self::BendSw => Direction::South,
            Self::BendNe | Self::BendNw => Direction::North,
            _ => Direction::West,
        }
    }
}

#[derive(Debug)]
struct Loop {
    nodes: Vec<(usize, usize)>,
}

impl Loop {
    fn half_size(&self) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }

        self.nodes.len() / 2
    }
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn get_starting_point_tile_type(
    north: Option<&TileType>,
    south: Option<&TileType>,
    east: Option<&TileType>,
    west: Option<&TileType>,
) -> Option<TileType> {
    if let Some(north_tile) = north {
        if north_tile.has_south() {
            if let Some(south_tile) = south {
                if south_tile.has_north() {
                    return Some(TileType::PipeVertical);
                }
            }

            if let Some(east_tile) = east {
                if east_tile.has_west() {
                    return Some(TileType::BendNe);
                }
            }

            if let Some(west_tile) = west {
                if west_tile.has_east() {
                    return Some(TileType::BendNw);
                }
            }
        }
    }

    if let Some(south_tile) = south {
        if south_tile.has_north() {
            // already checked north-south, skip here

            if let Some(east_tile) = east {
                if east_tile.has_west() {
                    return Some(TileType::BendSe);
                }
            }

            if let Some(west_tile) = west {
                if west_tile.has_east() {
                    return Some(TileType::BendSw);
                }
            }
        }
    }

    if let Some(east_tile) = east {
        if east_tile.has_west() {
            if let Some(west_tile) = west {
                if west_tile.has_east() {
                    return Some(TileType::PipeHorizontal);
                }
            }
        }
    }

    None
}

fn compute_half_loop_size<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut starting_point: Option<(usize, usize)> = None;

    let mut map: Vec<Vec<TileType>> = lines
        .enumerate()
        .map(|(index_row, line)| {
            let tile_row: Vec<TileType> = line
                .chars()
                .enumerate()
                .map(|(index_column, char)| {
                    let tile = TileType::from_char(char);
                    if tile == TileType::StartingPosition {
                        starting_point = Some((index_row, index_column));
                    }

                    tile
                })
                .collect();

            tile_row
        })
        .collect();

    if starting_point.is_none() {
        return 0;
    }

    let starting_point: (usize, usize) = starting_point.unwrap();

    log(format!("Starting Point {starting_point:?}").as_str());

    let north = if starting_point.0 > 0 {
        map.get(starting_point.0 - 1)
            .and_then(|row| row.get(starting_point.1).or(None))
    } else {
        None
    };

    let south = if starting_point.0 < map.len() {
        map.get(starting_point.0 + 1)
            .and_then(|row| row.get(starting_point.1).or(None))
    } else {
        None
    };

    let west = if starting_point.1 > 0 {
        map.get(starting_point.0)
            .and_then(|row| row.get(starting_point.1 - 1).or(None))
    } else {
        None
    };

    let east = if starting_point.1 < map.get(0).unwrap().len() {
        map.get(starting_point.0)
            .and_then(|row| row.get(starting_point.1 + 1).or(None))
    } else {
        None
    };

    let new_starting_tile_type = get_starting_point_tile_type(north, south, east, west);

    if new_starting_tile_type.is_none() {
        return 0;
    }

    log(format!("Starting Tile {new_starting_tile_type:?}").as_str());

    map[starting_point.0][starting_point.1] = new_starting_tile_type.unwrap();

    let map = map;

    let mut loop_vec: Vec<(usize, usize)> = vec![starting_point];

    let mut direction: Direction = map[starting_point.0][starting_point.1].initial_direction();

    loop {
        let current_node_index = loop_vec.last().unwrap();

        log(format!("Current Node Index {current_node_index:?}").as_str());

        if loop_vec.len() > 1 && *current_node_index == starting_point {
            break;
        }

        let current_node = map
            .get(current_node_index.0)
            .and_then(|row| row.get(current_node_index.1).or(None));

        if current_node.is_none() {
            log("Invalid previous step");

            return 0;
        }

        let current_node = current_node.unwrap();

        log(format!("Current Node Tile {current_node:?}").as_str());

        log(format!("Direction Entry {direction:?}").as_str());

        let new_direction = current_node.next(&direction);

        if new_direction.is_none() {
            log("Invalid step");

            return 0;
        }

        let new_direction = new_direction.unwrap();

        match new_direction {
            Direction::North => loop_vec.push((current_node_index.0 - 1, current_node_index.1)),
            Direction::South => loop_vec.push((current_node_index.0 + 1, current_node_index.1)),
            Direction::East => loop_vec.push((current_node_index.0, current_node_index.1 + 1)),
            Direction::West => loop_vec.push((current_node_index.0, current_node_index.1 - 1)),
        }

        direction = new_direction.opposite();
    }

    let built_loop = Loop { nodes: loop_vec };

    built_loop.half_size()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", compute_half_loop_size(input.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let result = compute_half_loop_size(lines.lines());

        assert_eq!(result, 4);

        let lines = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..";

        let result = compute_half_loop_size(lines.lines());

        assert_eq!(result, 8);
    }
}
