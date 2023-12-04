use log::debug;

#[derive(Clone, Default, Debug)]
struct Number {
    digits: String,
    valid: bool,
}

impl Number {
    fn add_digit(&mut self, digit: char) {
        self.digits.push(digit);
    }
}

fn log(message: &str) {
    if cfg!(feature = "debug") {
        debug!("{}", message);
    }
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn calculate_engine_part_id_sum<'a>(lines: impl Iterator<Item = &'a str>) -> u32 {
    let schematic: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    if schematic.is_empty() {
        return 0;
    }

    let mut part_numbers: Vec<Number> = Vec::default();

    let line_length = schematic.first().unwrap().len();

    let mut current_number: Option<Number> = None;

    let mut number_would_be_valid = false;

    for (index_line, line) in schematic.iter().enumerate() {
        if current_number.is_some() {
            let mut number = current_number.clone().unwrap();

            number.valid = number.valid || number_would_be_valid;

            if !number.digits.is_empty() {
                part_numbers.push(number);
            }

            current_number = None;
        }

        for (index_ch, ch) in line.iter().enumerate() {
            if current_number.is_some() {
                let mut number = current_number.clone().unwrap();
                number.valid = number.valid || number_would_be_valid;

                current_number = Some(number);
            }

            match ch {
                '.' => {
                    if current_number.is_some() {
                        let number = current_number.clone().unwrap();

                        if !number.digits.is_empty() {
                            part_numbers.push(number);
                        }

                        current_number = None;
                    }
                }
                _ => {
                    if ch.is_numeric() {
                        if current_number.is_none() {
                            current_number = Some(Number::default());
                        }

                        let mut number = current_number.clone().unwrap();
                        number.add_digit(*ch);

                        current_number = Some(number);
                    } else if current_number.is_some() {
                        let number = current_number.clone().unwrap();

                        if !number.digits.is_empty() {
                            part_numbers.push(current_number.unwrap());
                        }

                        current_number = None;
                    }
                }
            }

            let lines_to_check: Vec<usize> = match index_line {
                0 => vec![index_line, index_line + 1],
                _ => {
                    if index_line == schematic.len() {
                        vec![index_line - 1, index_line]
                    } else {
                        vec![index_line - 1, index_line, index_line + 1]
                    }
                }
            };

            let columns_to_check: Vec<usize> = match index_ch {
                0 => vec![index_ch, index_ch + 1],
                _ => {
                    if index_line == line_length {
                        vec![index_ch - 1, index_ch]
                    } else {
                        vec![index_ch - 1, index_ch, index_ch + 1]
                    }
                }
            };

            log(format!("Checking Char {}:", &ch).as_str());

            if *ch == '6' {
                log(format!("Lines: {:?}", &lines_to_check).as_str());
                log(format!("Columns: {:?}", &columns_to_check).as_str());
            }

            number_would_be_valid = {
                let mut is_valid = false;

                for i in lines_to_check {
                    if let Some(row) = schematic.get(i) {
                        for j in &columns_to_check {
                            if let Some(adjacent) = row.get(*j) {
                                log(format!("Checking {adjacent}").as_str());
                                if *adjacent != '.' && !adjacent.is_numeric() {
                                    is_valid = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                is_valid
            }
        }
    }

    if let Some(number) = current_number {
        if !number.digits.is_empty() {
            part_numbers.push(number);
        }
    }

    log(format!("{:?}", &part_numbers).as_str());

    part_numbers
        .iter()
        .filter(|number| number.valid)
        .filter_map(|number| number.digits.parse::<u32>().ok())
        .sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", calculate_engine_part_id_sum(input.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .lines();

        let result = calculate_engine_part_id_sum(lines);

        assert_eq!(result, 4361);
    }
}
