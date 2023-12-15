use log::debug;

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

#[derive(Clone, Debug)]
enum LensType {
    Replacement,
    Removal,
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: Option<u8>,
    lens_type: LensType,
}

fn parse_lens(lens: &str) -> Option<Lens> {
    let lens_type = if lens.contains('=') {
        LensType::Replacement
    } else if lens.contains('-') {
        LensType::Removal
    } else {
        return None;
    };

    let (label, _) = match lens_type {
        LensType::Replacement => lens.split_once('=').unwrap(),
        LensType::Removal => lens.split_once('-').unwrap(),
    };

    let label = label.to_string();

    let focal_length = match lens_type {
        LensType::Replacement => {
            let digits: String = lens.chars().filter(|c| c.is_numeric()).collect();
            digits.parse::<u8>().ok()
        }
        LensType::Removal => None,
    };

    Some(Lens {
        label,
        focal_length,
        lens_type,
    })
}

fn hash(sequence: &str) -> u32 {
    log(format!("Hashing {sequence}"));

    sequence
        .chars()
        .fold(0, |accumulator, c| ((accumulator + (c as u32)) * 17) % 256)
}

#[allow(clippy::cast_possible_truncation)]
fn calculate_focusing_power(input: &str) -> u32 {
    log(format!("Summing hashes of {input}"));

    let lenses: Vec<(u32, Lens)> = input
        .replace('\n', "")
        .split(',')
        .filter_map(parse_lens)
        .map(|lens| (hash(&lens.label), lens))
        .collect();

    let mut sum = 0;

    for n in 0..256 {
        let mut box_of_lenses: Vec<Lens> = Vec::default();

        for (_, lens) in lenses.iter().filter(|lens| lens.0 as usize == n).cloned() {
            match lens.lens_type {
                LensType::Replacement => {
                    if let Some(index) = box_of_lenses
                        .iter()
                        .position(|item| item.label == lens.label)
                    {
                        box_of_lenses[index] = lens;
                    } else {
                        box_of_lenses.push(lens);
                    }
                }
                LensType::Removal => {
                    if let Some(index) = box_of_lenses
                        .iter()
                        .position(|item| item.label == lens.label)
                    {
                        box_of_lenses.remove(index);
                    }
                }
            }
        }

        log(format!("Box of lenses {n} {box_of_lenses:?}"));

        sum += box_of_lenses
            .iter()
            .enumerate()
            .map(|(index_lens, lens)| {
                (1 + n as u32) * (1 + index_lens as u32) * u32::from(lens.focal_length.unwrap())
            })
            .sum::<u32>();
    }

    sum
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", calculate_focusing_power(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = calculate_focusing_power(lines);

        assert_eq!(result, 145);
    }

    #[test]
    fn test_hash() {
        let line = "rn=1";

        let result = hash(line);

        assert_eq!(result, 30);

        let line = "cm-";

        let result = hash(line);

        assert_eq!(result, 253);

        let line = "qp=3";

        let result = hash(line);

        assert_eq!(result, 97);
    }
}
