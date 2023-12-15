use log::debug;

fn log<S: AsRef<str>>(message: S) {
    if cfg!(feature = "debug") {
        debug!("{:?}", message.as_ref());
    }
}

fn hash(sequence: &str) -> u32 {
    log(format!("Hashing {sequence}"));

    sequence
        .chars()
        .fold(0, |accumulator, c| ((accumulator + (c as u32)) * 17) % 256)
}

fn sum_hashes(input: &str) -> u32 {
    log(format!("Summing hashes of {input}"));

    input.replace('\n', "").split(',').map(hash).sum()
}

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    let input = include_str!("../../input");

    println!("{}", sum_hashes(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let lines = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = sum_hashes(lines);

        assert_eq!(result, 1320);
    }

    #[test]
    fn test_halt() {
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
