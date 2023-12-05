use log::debug;

#[derive(Debug, PartialEq)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range: u64,
}

impl Map {
    const fn get(&self, value: u64) -> Option<u64> {
        if value >= self.source_range_start && value <= self.source_range_start + self.range {
            Some(self.destination_range_start + (value - self.source_range_start))
        } else {
            None
        }
    }
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

fn parse_line(line: &str) -> Option<Map> {
    log(format!("Parsing line: {line}").as_str());

    let line_values: Vec<u64> = line
        .split_whitespace()
        .filter_map(|value| value.parse::<u64>().ok())
        .collect();

    if line_values.len() < 3 {
        return None;
    }

    Some(Map {
        destination_range_start: *line_values.first().unwrap(),
        source_range_start: *line_values.get(1).unwrap(),
        range: *line_values.get(2).unwrap(),
    })
}

#[allow(clippy::too_many_lines)]
fn find_lowest_location_number(lines: &str) -> u64 {
    let groupings: Vec<&str> = lines.split("\n\n").collect();

    log(format!("Groupings: {groupings:?}").as_str());

    let seeds: Vec<(u64, u64)> = groupings
        .first()
        .unwrap()
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    log(format!("Seeds: {seeds:?}").as_str());

    let map_seed_to_soil: Vec<Map> = groupings
        .get(1)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Seed to Soil: {map_seed_to_soil:?}").as_str());

    let map_soil_to_fertiliser: Vec<Map> = groupings
        .get(2)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Soil to Fertiliser: {map_soil_to_fertiliser:?}").as_str());

    let map_fertiliser_to_water: Vec<Map> = groupings
        .get(3)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Fertiliser to Water: {map_fertiliser_to_water:?}").as_str());

    let map_water_to_light: Vec<Map> = groupings
        .get(4)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Water to Light: {map_water_to_light:?}").as_str());

    let map_light_to_temperature: Vec<Map> = groupings
        .get(5)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Light to Temperature: {map_light_to_temperature:?}").as_str());

    let map_temperature_to_humidity: Vec<Map> = groupings
        .get(6)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Temperature to Humidity: {map_temperature_to_humidity:?}").as_str());

    let map_humidity_to_location: Vec<Map> = groupings
        .get(7)
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(parse_line)
        .collect();

    log(format!("Humidity to Location: {map_humidity_to_location:?}").as_str());

    let mut locations: Vec<u64> = seeds
        .iter()
        .flat_map(|(seed, range)| {
            let all_location_values: Vec<u64> = (*seed..(seed + range))
                .flat_map(|n| {
                    log(format!("Seed: {seed}").as_str());

                    let mut soil_values: Vec<u64> = map_seed_to_soil
                        .iter()
                        .filter_map(|map| map.get(n))
                        .collect();

                    if soil_values.is_empty() {
                        soil_values = vec![n];
                    }

                    log(format!("Soil: {:?}", &soil_values).as_str());

                    let mut fertiliser_values: Vec<u64> = map_soil_to_fertiliser
                        .iter()
                        .flat_map(|map| soil_values.iter().map(|value| map.get(*value)))
                        .flatten()
                        .collect();

                    if fertiliser_values.is_empty() {
                        fertiliser_values = soil_values;
                    }

                    log(format!("Fertiliser: {:?}", &fertiliser_values).as_str());

                    let mut water_values: Vec<u64> = map_fertiliser_to_water
                        .iter()
                        .flat_map(|map| fertiliser_values.iter().map(|value| map.get(*value)))
                        .flatten()
                        .collect();

                    if water_values.is_empty() {
                        water_values = fertiliser_values;
                    }

                    log(format!("Water: {:?}", &water_values).as_str());

                    let mut light_values: Vec<u64> = map_water_to_light
                        .iter()
                        .flat_map(|map| water_values.iter().map(|value| map.get(*value)))
                        .flatten()
                        .collect();

                    if light_values.is_empty() {
                        light_values = water_values;
                    }

                    log(format!("Light: {:?}", &light_values).as_str());

                    let mut temperature_values: Vec<u64> = map_light_to_temperature
                        .iter()
                        .flat_map(|map| light_values.iter().map(|value| map.get(*value)))
                        .flatten()
                        .collect();

                    if temperature_values.is_empty() {
                        temperature_values = light_values;
                    }

                    log(format!("Temperature: {:?}", &temperature_values).as_str());

                    let mut humidity_values: Vec<u64> = map_temperature_to_humidity
                        .iter()
                        .flat_map(|map| temperature_values.iter().map(|value| map.get(*value)))
                        .flatten()
                        .collect();

                    if humidity_values.is_empty() {
                        humidity_values = temperature_values;
                    }

                    log(format!("Humidity: {:?}", &humidity_values).as_str());

                    let mut location_values: Vec<u64> = map_humidity_to_location
                        .iter()
                        .flat_map(|map| humidity_values.iter().map(|value| map.get(*value)))
                        .flatten()
                        .collect();

                    if location_values.is_empty() {
                        location_values = humidity_values;
                    }

                    log(format!("Location: {:?}", &location_values).as_str());

                    location_values
                })
                .collect();

            all_location_values
        })
        .collect();

    locations.sort_unstable();

    *locations.first().unwrap()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", find_lowest_location_number(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = find_lowest_location_number(lines);

        assert_eq!(result, 46);
    }
}
