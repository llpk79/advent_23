use std::fs::read_to_string;

pub fn hash(input: &str) -> u32 {
    let mut output = 0;
    for char in input.chars() {
        output += u32::from(char);
        output *= 17;
        output %= 256;
    }
    output
}

pub fn part_1() -> Option<()> {
    let input = read_to_string("src/day_15/input.txt").expect("file exists");
    let answer: u32 = input.split(',').map(hash).sum();
    println!("answer {answer}");
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp"), 1);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("pc"), 3);
    }
}
